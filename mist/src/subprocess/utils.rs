use super::MistServerService;
use crate::{service::MistServiceUtils, types::*};

// ISteamUtils
impl MistServiceUtils for MistServerService {
    fn get_appid(&mut self) -> AppId {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetAppID(self.steam_utils) }
    }
    fn is_steam_running_on_steam_deck(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(self.steam_utils) }
    }
}
