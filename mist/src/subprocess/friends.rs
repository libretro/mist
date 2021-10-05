use std::ffi::CString;

use super::MistServerService;
use crate::service::MistServiceFriends;

// ISteamFriends
impl MistServiceFriends for MistServerService {
    fn clear_rich_presence(&mut self) {
        unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_ClearRichPresence(self.steam_friends);
        }
    }
    fn set_rich_presence(&mut self, key: String, value: Option<String>) -> bool {
        // Turn the string into a c null terminated string
        let c_key = CString::new(key).unwrap_or_default();
        // value can be None (NULL) to clear it
        let c_value = value.map(|val| CString::new(val).ok()).flatten();

        unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_SetRichPresence(
                self.steam_friends,
                c_key.as_ptr() as *const _,
                // Get the ptr to the str if it has a value, otherwise return null
                c_value
                    .map(|v| v.into_raw() as *const _)
                    .unwrap_or(std::ptr::null()),
            )
        }
    }
}
