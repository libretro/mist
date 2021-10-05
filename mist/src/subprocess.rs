use anyhow::Result;
use std::time::Duration;

use crate::service::*;

pub fn run() -> Result<()> {
    // Setup the service context which is avaliable to all the service calls
    let service = MistServerService {
        steam_apps: unsafe { steamworks_sys::SteamAPI_SteamApps_v008() },
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
    steam_apps: *mut steamworks_sys::ISteamApps,
    steam_pipe: steamworks_sys::HSteamPipe,
    steam_friends: *mut steamworks_sys::ISteamFriends,
    steam_utils: *mut steamworks_sys::ISteamUtils,
    should_exit: bool,
}

mod apps;
mod friends;
mod utils;

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
