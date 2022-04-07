use raw_sync::locks::*;
use shared_memory::{Shmem, ShmemConf};
use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_ushort},
    sync::atomic::{AtomicU8, Ordering},
};

use super::MistServerService;
use crate::{
    consts::*,
    result::{Error, SteamInputError},
    service::MistServiceSteamInput,
    types::*,
};

pub struct SteamInputData {
    shmem: Shmem,
    state: MistInputState,
    last_counter: u8,
    analog_actions: Vec<MistInputAnalogActionHandle>,
    digital_actions: Vec<MistInputDigitalActionHandle>,
    lock: Box<dyn LockImpl>,
}

impl SteamInputData {
    pub fn new() -> Result<Self, Error> {
        let shmem = match ShmemConf::new().size(MistInputState::shmem_size()).create() {
            Ok(shmem) => shmem,
            Err(err) => {
                eprintln!("[mist] Error setting up shmem: {}", err);
                return Err(Error::SteamInput(SteamInputError::ShmemError));
            }
        };

        let raw_ptr = shmem.as_ptr();
        let counter_ptr: *mut AtomicU8 =
            unsafe { raw_ptr.add(Mutex::size_of(Some(raw_ptr))) } as *mut AtomicU8;
        let state_ptr =
            unsafe { raw_ptr.add(Mutex::size_of(Some(raw_ptr)) + std::mem::size_of::<AtomicU8>()) };

        let (lock, _bytes_used) = match unsafe { Mutex::new(raw_ptr, state_ptr) } {
            Ok(l) => l,
            Err(err) => {
                eprintln!("[mist] Error creating shmem mutex: {}", err);
                return Err(Error::SteamInput(SteamInputError::ShmemError));
            }
        };
        let state_ptr = state_ptr as *mut MistInputState;
        unsafe { *state_ptr = MistInputState::default() };
        unsafe { *counter_ptr = AtomicU8::new(0) };

        Ok(SteamInputData {
            shmem,
            state: MistInputState::default(),
            last_counter: 0,
            analog_actions: Vec::new(),
            digital_actions: Vec::new(),
            lock,
        })
    }

    pub fn os_id(&self) -> String {
        self.shmem.get_os_id().to_owned()
    }

    fn get_counter(&self) -> u8 {
        let raw_ptr = self.shmem.as_ptr();
        let counter_ptr: *mut AtomicU8 =
            unsafe { raw_ptr.add(Mutex::size_of(Some(raw_ptr))) } as *mut AtomicU8;
        unsafe { &*counter_ptr }.load(Ordering::Relaxed)
    }

    pub fn run_frame(&mut self, steam_input: *mut steamworks_sys::ISteamInput) {
        // Do not poll input until the library has processed the last one so we do not drop input
        if self.last_counter == self.get_counter() {
            return;
        }

        unsafe { steamworks_sys::SteamAPI_ISteamInput_RunFrame(steam_input, true) };

        self.state.input_handle_count = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetConnectedControllers(
                steam_input,
                &mut self.state.input_handles as *mut _,
            )
        };

        let input_handles = &self.state.input_handles[..self.state.input_handle_count as usize];

        // Remove gamepads no longer connected
        for handle in self.state.gamepad_mapping.iter_mut() {
            if *handle != 0 && !input_handles.contains(handle) {
                *handle = 0;
            }
        }

        // Add gamepads not mapped
        for handle in input_handles {
            if !self.state.gamepad_mapping.contains(handle) {
                if let Some(free_pos) = self.state.gamepad_mapping.iter().position(|h| *h == 0) {
                    self.state.gamepad_mapping[free_pos] = *handle;
                }
            }
        }

        for i in 0..MIST_STEAM_INPUT_MAX_COUNT {
            let input_handle = self.state.gamepad_mapping[i];

            if input_handle == 0 {
                continue;
            }

            let pad = &mut self.state.gamepads[i];

            let input_type = unsafe {
                std::mem::transmute::<_, MistSteamInputType>(
                    steamworks_sys::SteamAPI_ISteamInput_GetInputTypeForHandle(
                        steam_input,
                        input_handle,
                    ),
                )
            };

            pad.input_handle = input_handle;
            pad.input_type = input_type;

            // Only update actions if the controller is valid
            if pad.input_type != MistSteamInputType::Unknown {
                for analog_handle in &self.analog_actions {
                    let idx = *analog_handle as usize;
                    let analog_action_data = unsafe {
                        steamworks_sys::SteamAPI_ISteamInput_GetAnalogActionData(
                            steam_input,
                            input_handle,
                            *analog_handle,
                        )
                    };

                    pad.analog_action_data[idx] = MistInputAnalogActionData {
                        mode: unsafe {
                            std::mem::transmute::<_, MistControllerSourceMode>(
                                analog_action_data.eMode,
                            )
                        },
                        x: analog_action_data.x,
                        y: analog_action_data.y,
                        active: analog_action_data.bActive,
                    };
                }

                for digital_handle in &self.digital_actions {
                    let idx = *digital_handle as usize;
                    let digital_action_data = unsafe {
                        steamworks_sys::SteamAPI_ISteamInput_GetDigitalActionData(
                            steam_input,
                            input_handle,
                            *digital_handle,
                        )
                    };

                    pad.digital_action_data[idx] = MistInputDigitalActionData {
                        state: digital_action_data.bState,
                        active: digital_action_data.bActive,
                    };
                }

                let motion_data = unsafe {
                    steamworks_sys::SteamAPI_ISteamInput_GetMotionData(steam_input, input_handle)
                };

                pad.motion_data =
                    unsafe { std::mem::transmute::<_, MistInputMotionData>(motion_data) };
            }
        }

        // Update the state in the shared memory
        let raw_ptr = self.shmem.as_ptr();
        let counter_ptr: *mut AtomicU8 =
            unsafe { raw_ptr.add(Mutex::size_of(Some(raw_ptr))) } as *mut AtomicU8;
        let state_ptr =
            unsafe { raw_ptr.add(Mutex::size_of(Some(raw_ptr)) + std::mem::size_of::<AtomicU8>()) };

        let state_ptr = state_ptr as *mut MistInputState;

        let guard = self.lock.lock().unwrap();

        // Copy the state
        unsafe { *state_ptr = self.state };
        self.last_counter = unsafe { &*counter_ptr }.load(Ordering::Relaxed);

        drop(guard);
    }
}

// ISteamInput
impl MistServiceSteamInput for MistServerService {
    fn activate_action_set(
        &mut self,
        input_handle: MistInputHandle,
        action_set_handle: MistInputActionSetHandle,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_ActivateActionSet(
                self.steam_input,
                input_handle,
                action_set_handle,
            )
        }

        Ok(())
    }
    fn activate_action_set_layer(
        &mut self,
        input_handle: MistInputHandle,
        action_set_layer_handle: MistInputActionSetHandle,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_ActivateActionSetLayer(
                self.steam_input,
                input_handle,
                action_set_layer_handle,
            )
        }
        Ok(())
    }
    fn deactivate_action_set_layer(
        &mut self,
        input_handle: MistInputHandle,
        action_set_layer_handle: MistInputActionSetHandle,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_DeactivateActionSetLayer(
                self.steam_input,
                input_handle,
                action_set_layer_handle,
            )
        }
        Ok(())
    }
    fn deactivate_all_action_set_layers(
        &mut self,
        input_handle: MistInputHandle,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_DeactivateAllActionSetLayers(
                self.steam_input,
                input_handle,
            )
        }
        Ok(())
    }
    fn get_active_action_set_layers(
        &mut self,
        input_handle: MistInputHandle,
    ) -> Result<Vec<MistInputActionSetHandle>, Error> {
        let mut active_action_set_layers: [MistInputActionSetHandle;
            steamworks_sys::STEAM_INPUT_MAX_COUNT as usize] = Default::default();

        let handles = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetActiveActionSetLayers(
                self.steam_input,
                input_handle,
                &mut active_action_set_layers as *mut _,
            )
        };

        Ok(active_action_set_layers[..handles as usize].to_vec())
    }
    fn get_action_set_handle(
        &mut self,
        action_set_name: String,
    ) -> Result<MistInputActionSetHandle, Error> {
        let name_cstr = CString::new(action_set_name).unwrap_or_default();

        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetActionSetHandle(
                self.steam_input,
                name_cstr.as_ptr() as *const _,
            )
        })
    }
    fn get_analog_action_handle(
        &mut self,
        name: String,
    ) -> Result<MistInputAnalogActionHandle, Error> {
        let name_cstr = CString::new(name).unwrap_or_default();

        let handle = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetAnalogActionHandle(
                self.steam_input,
                name_cstr.as_ptr() as *const _,
            )
        };

        if let Some(input_data) = &mut self.steam_input_data {
            if !input_data.analog_actions.contains(&handle) {
                input_data.analog_actions.push(handle);
            }
        }

        Ok(handle)
    }
    fn get_analog_action_origins(
        &mut self,
        input_handle: MistInputHandle,
        action_set_handle: MistInputActionSetHandle,
        analog_action_handle: MistInputAnalogActionHandle,
    ) -> Result<Vec<MistInputActionOrigin>, Error> {
        let mut origins: [steamworks_sys::EInputActionOrigin;
            steamworks_sys::STEAM_INPUT_MAX_ORIGINS as usize] =
            [0; steamworks_sys::STEAM_INPUT_MAX_ORIGINS as usize];

        let origins_count = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetAnalogActionOrigins(
                self.steam_input,
                input_handle,
                action_set_handle,
                analog_action_handle,
                &mut origins as *mut _,
            )
        };

        Ok(origins[..origins_count as usize]
            .iter()
            .map(|origin| unsafe {
                std::mem::transmute::<_, MistInputActionOrigin>(*origin as u32)
            })
            .collect())
    }
    fn get_connected_controllers(&mut self) -> Result<Vec<MistInputHandle>, Error> {
        let mut input_handles: [MistInputHandle; steamworks_sys::STEAM_INPUT_MAX_COUNT as usize] =
            Default::default();

        let handles = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetConnectedControllers(
                self.steam_input,
                &mut input_handles as *mut _,
            )
        };

        Ok(input_handles[..handles as usize].to_vec())
    }
    fn get_controller_for_gamepad_index(&mut self, index: c_int) -> Result<MistInputHandle, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetControllerForGamepadIndex(
                self.steam_input,
                index,
            )
        })
    }
    fn get_current_action_set(
        &mut self,
        input_handle: MistInputHandle,
    ) -> Result<MistInputActionSetHandle, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetCurrentActionSet(self.steam_input, input_handle)
        })
    }
    fn get_digital_action_handle(
        &mut self,
        name: String,
    ) -> Result<MistInputDigitalActionHandle, Error> {
        let name_cstr = CString::new(name).unwrap_or_default();

        let handle = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetDigitalActionHandle(
                self.steam_input,
                name_cstr.as_ptr() as *const _,
            )
        };

        if let Some(input_data) = &mut self.steam_input_data {
            if !input_data.digital_actions.contains(&handle) {
                input_data.digital_actions.push(handle);
            }
        }

        Ok(handle)
    }
    fn get_digital_action_origins(
        &mut self,
        input_handle: MistInputHandle,
        action_set_handle: MistInputActionSetHandle,
        digital_action_handle: MistInputDigitalActionHandle,
    ) -> Result<Vec<MistInputActionOrigin>, Error> {
        let mut origins: [steamworks_sys::EInputActionOrigin;
            steamworks_sys::STEAM_INPUT_MAX_ORIGINS as usize] =
            [0; steamworks_sys::STEAM_INPUT_MAX_ORIGINS as usize];

        let origins_count = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetDigitalActionOrigins(
                self.steam_input,
                input_handle,
                action_set_handle,
                digital_action_handle,
                &mut origins as *mut _,
            )
        };

        Ok(origins[..origins_count as usize]
            .iter()
            .map(|origin| unsafe {
                std::mem::transmute::<_, MistInputActionOrigin>(*origin as u32)
            })
            .collect())
    }
    fn get_gamepad_index_for_controller(
        &mut self,
        controller_handle: MistInputHandle,
    ) -> Result<c_int, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetGamepadIndexForController(
                self.steam_input,
                controller_handle,
            )
        })
    }
    fn get_glyph_png_for_action_origin(
        &mut self,
        origin: MistInputActionOrigin,
        size: MistSteamInputGlyphSize,
        flags: MistSteamInputGlyphStyle,
    ) -> Result<CString, Error> {
        let glyph_ptr = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetGlyphPNGForActionOrigin(
                self.steam_input,
                origin as _,
                size as _,
                flags as _,
            )
        };

        assert!(!glyph_ptr.is_null());

        let glyph_path = unsafe { CStr::from_ptr(glyph_ptr) }.to_owned();
        Ok(glyph_path)
    }
    fn get_glyph_svg_for_action_origin(
        &mut self,
        origin: MistInputActionOrigin,
        flags: MistSteamInputGlyphStyle,
    ) -> Result<CString, Error> {
        let glyph_ptr = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetGlyphSVGForActionOrigin(
                self.steam_input,
                origin as _,
                flags as _,
            )
        };

        assert!(!glyph_ptr.is_null());

        let glyph_path = unsafe { CStr::from_ptr(glyph_ptr) }.to_owned();
        Ok(glyph_path)
    }
    fn get_input_type_for_handle(
        &mut self,
        input_handle: MistInputHandle,
    ) -> Result<MistSteamInputType, Error> {
        Ok(unsafe {
            std::mem::transmute::<_, MistSteamInputType>(
                steamworks_sys::SteamAPI_ISteamInput_GetInputTypeForHandle(
                    self.steam_input,
                    input_handle,
                ),
            )
        })
    }
    fn get_string_for_action_origin(
        &mut self,
        origin: MistInputActionOrigin,
    ) -> Result<CString, Error> {
        let action_origin_string_ptr = unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetStringForActionOrigin(
                self.steam_input,
                origin as _,
            )
        };

        assert!(!action_origin_string_ptr.is_null());

        let action_origin_string = unsafe { CStr::from_ptr(action_origin_string_ptr) }.to_owned();

        Ok(action_origin_string)
    }
    fn init(&mut self) -> Result<(String, bool), Error> {
        let input_data = SteamInputData::new()?;
        let os_id = input_data.os_id();

        let succ = unsafe { steamworks_sys::SteamAPI_ISteamInput_Init(self.steam_input, true) };

        if succ {
            self.steam_input_data = Some(input_data);
        }

        Ok((os_id, succ))
    }
    fn set_input_action_manifest_file_path(&mut self, path: CString) -> Result<bool, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_SetInputActionManifestFilePath(
                self.steam_input,
                path.as_ptr(),
            )
        })
    }
    fn set_led_color(
        &mut self,
        input_handle: MistInputHandle,
        color_r: u8,
        color_g: u8,
        color_b: u8,
        flags: MistSteamControllerLEDFlag,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_SetLEDColor(
                self.steam_input,
                input_handle,
                color_r,
                color_g,
                color_b,
                flags as _,
            );
        }

        Ok(())
    }
    // fn show_analog_action_origins... Deprecated so not implemented
    fn show_binding_panel(&mut self, input_handle: MistInputHandle) -> Result<bool, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_ShowBindingPanel(self.steam_input, input_handle)
        })
    }
    // fn show_digital_action_origins... Deprecated so not implemented
    fn shutdown(&mut self) -> Result<bool, Error> {
        Ok(unsafe { steamworks_sys::SteamAPI_ISteamInput_Shutdown(self.steam_input) })
    }
    fn stop_analog_action_momentum(
        &mut self,
        input_handle: MistInputHandle,
        action: MistInputAnalogActionHandle,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_StopAnalogActionMomentum(
                self.steam_input,
                input_handle,
                action,
            );
        }
        Ok(())
    }
    fn trigger_vibration(
        &mut self,
        input_handle: MistInputHandle,
        left_speed: c_ushort,
        right_speed: c_ushort,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_TriggerVibration(
                self.steam_input,
                input_handle,
                left_speed,
                right_speed,
            );
        }
        Ok(())
    }
    fn trigger_vibration_extended(
        &mut self,
        input_handle: MistInputHandle,
        left_speed: c_ushort,
        right_speed: c_ushort,
        left_trigger_speed: c_ushort,
        right_trigger_speed: c_ushort,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_TriggerVibrationExtended(
                self.steam_input,
                input_handle,
                left_speed,
                right_speed,
                left_trigger_speed,
                right_trigger_speed,
            );
        }
        Ok(())
    }
    fn trigger_simple_haptic_event(
        &mut self,
        input_handle: MistInputHandle,
        haptic_location: MistControllerHapticLocation,
        intensity: u8,
        gain_db: c_char,
        other_intensity: u8,
        other_gain_db: c_char,
    ) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_TriggerSimpleHapticEvent(
                self.steam_input,
                input_handle,
                haptic_location as _,
                intensity,
                gain_db,
                other_intensity,
                other_gain_db,
            );
        }

        Ok(())
    }
    fn translate_action_origin(
        &mut self,
        destination_input_type: MistSteamInputType,
        source_origin: MistInputActionOrigin,
    ) -> Result<MistInputActionOrigin, Error> {
        Ok(unsafe {
            std::mem::transmute::<_, MistInputActionOrigin>(
                steamworks_sys::SteamAPI_ISteamInput_TranslateActionOrigin(
                    self.steam_input,
                    destination_input_type as _,
                    source_origin as _,
                ),
            )
        })
    }
}
