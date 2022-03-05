#[macro_use]
mod codegen;
pub mod callbacks;
mod consts;
pub mod result;
mod service;
#[macro_use]
mod lib_subprocess;
mod types;

use callbacks::MistCallbackMsg;
use result::{Error, MistError, MistResult, Success};

macro_rules! unwrap_client_result {
    ($res:expr) => {
        match $res {
            Ok(res) => res,
            Err(err) => {
                return err.into();
            }
        }
    };
}

pub fn mist_log_error(err: &str) {
    eprintln!("[mist] {}", err);
}

/// Init mist, this is throwns an error if it was already initialised
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_subprocess_init() -> MistResult {
    let result = std::panic::catch_unwind(lib_subprocess::mist_init_subprocess);

    match result {
        Ok(res) => unwrap_client_result!(res),
        Err(_) => {
            mist_log_error("Internal panic during initialization");
            return Error::Mist(MistError::SubprocessNotInitialized).into();
        }
    }

    Success
}

/// Polls the subprocess
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_poll() -> MistResult {
    let subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().poll());

    Success
}

/// Attempts to return the next callback, if none are left it will set p_callback to NULL
/// Safety: The pointer is only valid until the next call of this function
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_next_callback(
    has_callback: *mut bool,
    p_callback: *mut MistCallbackMsg,
) -> MistResult {
    static mut HAS_PROCESSED_CALLBACK: bool = false;

    let subprocess = get_subprocess!();

    // Get the callback queue
    let queue = subprocess.client().callbacks();

    // Remove the previous callback
    if unsafe { HAS_PROCESSED_CALLBACK } {
        queue.pop_front();
    } else {
        unsafe {
            HAS_PROCESSED_CALLBACK = true;
        }
    }

    // Null the callback ptr ptr if the queue is empty
    if let Some(front) = queue.front() {
        unsafe {
            *p_callback = MistCallbackMsg {
                user: front.user,
                callback: front.callback,
                data: &front.data as *const _ as *const std::ffi::c_void,
            };
            *has_callback = true;
        }
    } else {
        unsafe {
            *has_callback = false;
        }
    }

    Success
}

#[path = "../lib/apps.rs"]
mod apps;
#[path = "../lib/friends.rs"]
mod friends;
#[path = "../lib/remote_storage.rs"]
mod remote_storage;
#[path = "../lib/utils.rs"]
mod utils;

/// Deinits the mist subprocess, returns false on error
#[no_mangle]
pub extern "C" fn mist_subprocess_deinit() -> MistResult {
    unwrap_client_result!(lib_subprocess::mist_deinit_subprocess());

    Success
}

// Workaround for build process
#[cfg(feature = "mist-bin")]
mod subprocess;
