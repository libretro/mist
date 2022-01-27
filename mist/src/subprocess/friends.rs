use std::ffi::CString;

use super::MistServerService;
use crate::{result::Error, service::MistServiceFriends};

// ISteamFriends
impl MistServiceFriends for MistServerService {
    fn clear_rich_presence(&mut self) -> Result<(), Error> {
        unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_ClearRichPresence(self.steam_friends);
        }

        Ok(())
    }
    fn set_rich_presence(&mut self, key: String, value: Option<String>) -> Result<bool, Error> {
        // Turn the string into a c null terminated string
        let c_key = CString::new(key).unwrap_or_default();

        static mut PRESENCE_VALUE: Option<CString> = None;
        // value can be None (NULL) to clear it
        unsafe { PRESENCE_VALUE = value.map(|val| CString::new(val).ok()).flatten() }

        Ok(unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_SetRichPresence(
                self.steam_friends,
                c_key.as_ptr() as *const _,
                // Get the ptr to the str if it has a value, otherwise return null
                PRESENCE_VALUE
                    .as_ref()
                    .map(|v| v.as_ptr() as *const _)
                    .unwrap_or(std::ptr::null()),
            )
        })
    }
}
