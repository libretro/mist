use super::MistServerService;
use crate::{result::Error, service::MistServiceSteamUtils, types::*};

// ISteamUtils
impl MistServiceSteamUtils for MistServerService {
    fn get_appid(&mut self) -> Result<AppId, Error> {
        Ok(unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetAppID(self.steam_utils) })
    }
    fn is_steam_running_on_steam_deck(&mut self) -> Result<bool, Error> {
        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(self.steam_utils)
        })
    }
}
