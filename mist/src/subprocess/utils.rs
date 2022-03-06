use std::{ffi::CString, os::raw::c_int};
use steamworks_sys::*;

use super::MistServerService;
use crate::{result::Error, service::MistServiceSteamUtils, types::*};

// ISteamUtils
impl MistServiceSteamUtils for MistServerService {
    fn get_appid(&mut self) -> Result<AppId, Error> {
        Ok(unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetAppID(self.steam_utils) })
    }
    fn get_current_battery_power(&mut self) -> Result<u8, Error> {
        Ok(
            unsafe {
                steamworks_sys::SteamAPI_ISteamUtils_GetCurrentBatteryPower(self.steam_utils)
            },
        )
    }
    fn get_entered_gamepad_text_input(&mut self) -> Result<Option<String>, Error> {
        Ok(self.entered_gamepad_text.take())
    }
    fn is_overlay_enabled(&mut self) -> Result<bool, Error> {
        Ok(unsafe { steamworks_sys::SteamAPI_ISteamUtils_IsOverlayEnabled(self.steam_utils) })
    }
    fn is_steam_in_big_picture_mode(&mut self) -> Result<bool, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_IsSteamInBigPictureMode(self.steam_utils)
        })
    }
    fn is_steam_running_in_vr(&mut self) -> Result<bool, Error> {
        Ok(unsafe { steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningInVR(self.steam_utils) })
    }
    fn is_vr_headset_streaming_enabled(&mut self) -> Result<bool, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_IsVRHeadsetStreamingEnabled(self.steam_utils)
        })
    }
    fn is_steam_running_on_steam_deck(&mut self) -> Result<bool, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(self.steam_utils)
        })
    }
    fn set_vr_headset_streaming_enabled(&mut self, enabled: bool) -> Result<(), Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_SetVRHeadsetStreamingEnabled(
                self.steam_utils,
                enabled,
            )
        })
    }
    fn show_gamepad_text_input(
        &mut self,
        input_mode: MistGamepadTextInputMode,
        line_input_mode: MistGamepadTextInputLineMode,
        description: String,
        char_max: u32,
        existing_text: String,
    ) -> Result<bool, Error> {
        let input_mode = match input_mode {
            MistGamepadTextInputMode::Normal => EGamepadTextInputMode_k_EGamepadTextInputModeNormal,
            MistGamepadTextInputMode::Password => {
                EGamepadTextInputMode_k_EGamepadTextInputModePassword
            }
        };

        let line_input_mode = match line_input_mode {
            MistGamepadTextInputLineMode::SingleLine => {
                EGamepadTextInputLineMode_k_EGamepadTextInputLineModeSingleLine
            }
            MistGamepadTextInputLineMode::MultipleLines => {
                EGamepadTextInputLineMode_k_EGamepadTextInputLineModeMultipleLines
            }
        };

        let c_description = CString::new(description).unwrap_or_default();
        let c_existing_text = CString::new(existing_text).unwrap_or_default();

        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_ShowGamepadTextInput(
                self.steam_utils,
                input_mode,
                line_input_mode,
                c_description.as_ptr(),
                char_max,
                c_existing_text.as_ptr(),
            )
        })
    }
    fn show_floating_gamepad_text_input(
        &mut self,
        keyboard_mode: MistFloatingGamepadTextInputMode,
        text_field_x_position: c_int,
        text_field_y_position: c_int,
        text_field_width: c_int,
        text_field_height: c_int,
    ) -> Result<bool, Error> {
        let keyboard_mode = match keyboard_mode {
            MistFloatingGamepadTextInputMode::SingleLine => {
                EFloatingGamepadTextInputMode_k_EFloatingGamepadTextInputModeModeSingleLine
            }
            MistFloatingGamepadTextInputMode::MultipleLines => {
                EFloatingGamepadTextInputMode_k_EFloatingGamepadTextInputModeModeMultipleLines
            }
            MistFloatingGamepadTextInputMode::Email => {
                EFloatingGamepadTextInputMode_k_EFloatingGamepadTextInputModeModeEmail
            }
            MistFloatingGamepadTextInputMode::Numeric => {
                EFloatingGamepadTextInputMode_k_EFloatingGamepadTextInputModeModeNumeric
            }
        };

        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput(
                self.steam_utils,
                keyboard_mode,
                text_field_x_position,
                text_field_y_position,
                text_field_width,
                text_field_height,
            )
        })
    }
    fn set_game_launcher_mode(&mut self, launcher_mode: bool) -> Result<(), Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_SetGameLauncherMode(
                self.steam_utils,
                launcher_mode,
            )
        })
    }
    fn start_vr_dashboard(&mut self) -> Result<(), Error> {
        Ok(unsafe { steamworks_sys::SteamAPI_ISteamUtils_StartVRDashboard(self.steam_utils) })
    }
}
