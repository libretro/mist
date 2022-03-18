use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_int, c_ushort},
};

use super::MistServerService;
use crate::{result::Error, service::MistServiceSteamInput, types::*};

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
    // TODO: fn get_analog_action_data(input_handle: MistInputHandle, analog_action_handle: MistInputAnalogActionHandle) -> AnalogActionData;
    fn get_analog_action_handle(
        &mut self,
        name: String,
    ) -> Result<MistInputAnalogActionHandle, Error> {
        let name_cstr = CString::new(name).unwrap_or_default();

        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetAnalogActionHandle(
                self.steam_input,
                name_cstr.as_ptr() as *const _,
            )
        })
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
    // TODO: fn get_digital_action_data(input_handle: MistInputHandle, digital_action_handle: InputDigitalActionHandle) -> (&mut self, ) {}
    fn get_digital_action_handle(
        &mut self,
        name: String,
    ) -> Result<MistInputDigitalActionHandle, Error> {
        let name_cstr = CString::new(name).unwrap_or_default();

        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamInput_GetDigitalActionHandle(
                self.steam_input,
                name_cstr.as_ptr() as *const _,
            )
        })
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
    // TODO: fn get_motion_data(&mut self, input_handle: MistInputHandle) -> InputMotionData;
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
    fn init(&mut self) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_Init(self.steam_input, true);
        }

        Ok(())
    }
    // fn run_frame(&mut self, ) {} - Skipped and implemented on a higher level
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
    fn shutdown(&mut self) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamInput_Shutdown(self.steam_input);
        }

        Ok(())
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
