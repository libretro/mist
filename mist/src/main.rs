use anyhow::Result;
use std::{ffi::CString, time::Duration};

#[macro_use]
mod codegen;
mod consts;
mod service;
mod types;

use consts::PROCESS_INIT_SECRET;
use service::*;

fn main() {
    // Keep users away
    if std::env::args()
        .nth(1)
        .map(|e| e != PROCESS_INIT_SECRET)
        .unwrap_or(true)
    {
        eprintln!("This executable is a subprocess used for steam integration and is not designed to be launched manually.");
        std::process::exit(1);
    }

    // Init the steam api
    unsafe {
        if !steamworks_sys::SteamAPI_Init() {
            eprintln!("[mist] Error during SteamAPI init.");
            std::process::exit(1);
        }

        // Setup manual dispatch since we are not using c++ classes
        steamworks_sys::SteamAPI_ManualDispatch_Init();
    }

    if let Err(err) = run() {
        eprintln!("[mist] Error while running subprocess: {}", err);
    }

    std::process::exit(0);
}

fn run() -> Result<()> {
    // Setup the service context which is avaliable to all the service calls
    let service = MistServerService {
        steam_pipe: unsafe { steamworks_sys::SteamAPI_GetHSteamPipe() },
        steam_friends: unsafe { steamworks_sys::SteamAPI_SteamFriends_v017() },
        steam_utils: unsafe { steamworks_sys::SteamAPI_SteamUtils_v010() },
        should_exit: false,
    };

    // Create the server using stdin/stdout as transport for IPC
    let mut server = MistServer::create(service, std::io::stdin(), std::io::stdout());
    // Tell the library that we have initialized
    if let Err(err) = server.write_data(&MistServiceToLibrary::Initialized) {
        eprintln!(
            "[mist] Error writing intialized message to library: {}",
            err
        );
        std::process::exit(1);
    }

    while !server.service().should_exit {
        // Poll for messages from the library until 50ms timeout is reached
        server.recv_timeout(Duration::from_millis(50));

        // Run the frame
        unsafe {
            steamworks_sys::SteamAPI_ManualDispatch_RunFrame(server.service().steam_pipe);
        }
    }

    Ok(())
}

pub struct MistServerService {
    steam_pipe: steamworks_sys::HSteamPipe,
    steam_friends: *mut steamworks_sys::ISteamFriends,
    steam_utils: *mut steamworks_sys::ISteamUtils,
    should_exit: bool,
}

// Friends
impl MistServiceFriends for MistServerService {
    fn friends_clear_rich_presence(&mut self) {
        unsafe {
            steamworks_sys::SteamAPI_ISteamFriends_ClearRichPresence(self.steam_friends);
        }
    }
    fn friends_set_rich_presence(&mut self, key: String, value: Option<String>) -> bool {
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

// Utils
impl MistServiceUtils for MistServerService {
    fn utils_get_appid(&mut self) -> u32 {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetAppID(self.steam_utils) }
    }
    fn utils_is_steam_running_on_steam_deck(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(self.steam_utils) }
    }
}

impl MistServiceInternal for MistServerService {
    fn exit(&mut self) {
        self.should_exit = true;
    }
}

impl MistService for MistServerService {}

impl Drop for MistServerService {
    fn drop(&mut self) {
        // Shutdown the steam api
        unsafe { steamworks_sys::SteamAPI_Shutdown() };
    }
}
