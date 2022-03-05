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
