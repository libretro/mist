use std::{ffi::CString, time::Duration};

#[macro_use]
mod codegen;
mod consts;
mod service;

use consts::PROCESS_INIT_SECRET;
use service::{MistServer, MistService, MistServiceToLibrary};

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

    std::process::exit(0);
}

fn run() -> Result<(), String> {
    // Setup the service context which is avaliable to all the service calls
    let service = MistServerService {
        steam_pipe: unsafe { steamworks_sys::SteamAPI_GetHSteamPipe() },
        steam_friends: unsafe { steamworks_sys::SteamAPI_SteamFriends_v017() },
        steam_utils: unsafe { steamworks_sys::SteamAPI_SteamUtils_v010() },
    };

    // Create the server using stdin/stdout as transport for IPC
    let mut server = MistServer::create(service, std::io::stdin(), std::io::stdout());
    // Tell the library that we have initialized
    server.write_data(&MistServiceToLibrary::Initialized);

    loop {
        // Poll for messages from the library until 50ms timeout is reached
        server.recv_timeout(Duration::from_millis(50));

        // Run the frame
        unsafe {
            steamworks_sys::SteamAPI_ManualDispatch_RunFrame(server.service().steam_pipe);
        }
    }
}

pub struct MistServerService {
    steam_pipe: steamworks_sys::HSteamPipe,
    steam_friends: *mut steamworks_sys::ISteamFriends,
    steam_utils: *mut steamworks_sys::ISteamUtils,
}

impl MistService for MistServerService {
    // Friends
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
    // Utils
    fn get_appid(&mut self) -> u32 {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetAppID(self.steam_utils) }
    }
    fn is_steam_running_on_steam_deck(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(self.steam_utils) }
    }
}

impl Drop for MistServerService {
    fn drop(&mut self) {
        // Shutdown the steam api
        unsafe { steamworks_sys::SteamAPI_Shutdown() };
    }
}