use std::ffi::CString;

mod consts;
#[macro_use]
mod macros;
mod service;

use consts::PROCESS_INIT_SECRET;
use service::{MistServer, MistService};

fn main() {
    // Keep users away
    if std::env::args()
        .nth(1)
        .map(|e| e != PROCESS_INIT_SECRET)
        .unwrap_or(true)
    {
        println!("This executable is a subprocess used for steam integration and is not designed to be launched manually.");
        std::process::exit(1);
    }

    // Init the steam api
    unsafe {
        if !steamworks_sys::SteamAPI_Init() {
            todo!();
            //return Err("SteamAPI init failed.".into());
        }

        // Setup manual dispatch since we are not using c++ classes
        steamworks_sys::SteamAPI_ManualDispatch_Init();
    }

    if let Err(_err) = run() {}

    // Shutdown the steam api
    unsafe { steamworks_sys::SteamAPI_Shutdown() };

    std::process::exit(0);
}

fn run() -> Result<(), String> {
    // Setup the service context which is avaliable to all the service calls
    let service = MistServerService {
        steam_friends: unsafe { steamworks_sys::SteamAPI_SteamFriends_v017() },
        steam_utils: unsafe { steamworks_sys::SteamAPI_SteamUtils_v010() },
    };

    // Create the server using stdin/stdout as transport for IPC
    let server = MistServer::create(service, std::io::stdin(), std::io::stdout());

    Ok(())
}

pub struct MistServerService {
    steam_friends: *mut steamworks_sys::ISteamFriends,
    steam_utils: *mut steamworks_sys::ISteamUtils,
}

impl MistService for MistServerService {
    // Rich presence
    fn clear_rich_presence(&mut self) {
        unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_ClearRichPresence(self.steam_friends);
        }
    }
    fn set_rich_presence(&mut self, key: String, value: Option<String>) -> bool {
        // Turn the string into a c null terminated string
        let c_key = match CString::new(key) {
            Ok(cstr) => cstr,
            Err(_) => return false, // The string contains a null character which is illegal
        };

        // value can be None (NULL) to clear it
        let c_value = match value.map(|v| CString::new(v)) {
            Some(Ok(cstr)) => Some(cstr),
            Some(Err(_)) => return false, // The string contains a null character which is illegal
            None => None,
        };

        unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_SetRichPresence(
                self.steam_friends,
                c_key.as_ptr(),
                // Get the ptr to the str if it has a value, otherwise return null
                c_value.map(|v| v.as_ptr()).unwrap_or(std::ptr::null()),
            )
        }
    }
    // Utils
    fn get_appid(&mut self) -> u32 {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetAppID(self.steam_utils) }
    }
    fn is_steam_running_on_steam_deck(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(self.steam_utils) }
    }
}
