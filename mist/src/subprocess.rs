use anyhow::Result;
use std::time::Duration;

use crate::{result::Error, service::*};

pub type Server = MistServer<MistServerService, std::io::Stdin, std::io::Stdout>;

pub fn run() -> Result<()> {
    // Setup the service context which is avaliable to all the service calls
    let service = MistServerService {
        steam_apps: unsafe { steamworks_sys::SteamAPI_SteamApps_v008() },
        steam_pipe: unsafe { steamworks_sys::SteamAPI_GetHSteamPipe() },
        steam_friends: unsafe { steamworks_sys::SteamAPI_SteamFriends_v017() },
        steam_remote_storage: unsafe { steamworks_sys::SteamAPI_SteamRemoteStorage_v016() },
        steam_user: unsafe { steamworks_sys::SteamAPI_GetHSteamUser() },
        steam_utils: unsafe { steamworks_sys::SteamAPI_SteamUtils_v010() },
        entered_gamepad_text: None,
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

        let steam_pipe = server.service().steam_pipe;
        let steam_user = server.service().steam_user;

        // Run the frame
        unsafe {
            steamworks_sys::SteamAPI_ManualDispatch_RunFrame(steam_pipe);
        }

        let mut callback: steamworks_sys::CallbackMsg_t =
            unsafe { std::mem::MaybeUninit::zeroed().assume_init() };

        // Get callbacks and send the ones we want to the library
        while unsafe {
            steamworks_sys::SteamAPI_ManualDispatch_GetNextCallback(
                steam_pipe,
                &mut callback as *mut _,
            )
        } {
            if let Some(callback) = crate::callbacks::MistCallback::from_steam_callback(
                &mut server,
                steam_user,
                &callback,
            ) {
                if let Err(err) = server.write_data(&MistServiceToLibrary::Callback(callback)) {
                    eprintln!("[mist] Error writing callback message to library: {}", err);
                    std::process::exit(1);
                }
            }

            unsafe { steamworks_sys::SteamAPI_ManualDispatch_FreeLastCallback(steam_pipe) }
        }
    }

    Ok(())
}

pub struct MistServerService {
    steam_apps: *mut steamworks_sys::ISteamApps,
    steam_pipe: steamworks_sys::HSteamPipe,
    steam_friends: *mut steamworks_sys::ISteamFriends,
    steam_remote_storage: *mut steamworks_sys::ISteamRemoteStorage,
    steam_user: steamworks_sys::HSteamUser,
    steam_utils: *mut steamworks_sys::ISteamUtils,
    pub entered_gamepad_text: Option<String>,
    should_exit: bool,
}

mod apps;
mod friends;
mod remote_storage;
mod utils;

impl MistServiceInternal for MistServerService {
    fn exit(&mut self) -> Result<(), Error> {
        self.should_exit = true;

        Ok(())
    }
}

impl MistService for MistServerService {}

impl Drop for MistServerService {
    fn drop(&mut self) {
        // Shutdown the steam api
        unsafe { steamworks_sys::SteamAPI_Shutdown() };
    }
}
