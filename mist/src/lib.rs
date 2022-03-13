use std::{ffi::CString, os::raw::c_char};

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

pub unsafe fn copy_string_out(string: &CString, out: *mut c_char, size: usize) -> usize {
    // Also copy null byte
    let copied = (string.as_bytes().len() + 1).min(size as usize);

    std::ptr::copy_nonoverlapping(string.as_ptr() as *mut i8, out, copied);

    copied
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
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().poll());

    Success
}

/// Attempts to return the next callback, if none are left it will set p_callback to NULL
/// Safety: The pointer is only valid until the next call of this function
/// Due to this it is not safe to simultaneously access callbacks from two different threads since they might race invalidate the other threads callback
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_next_callback(
    has_callback: *mut bool,
    p_callback: *mut MistCallbackMsg,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let has_processed_callback = subprocess.state().has_processed_callback;

    // Get the callback queue
    let queue = subprocess.client().callbacks();

    if has_processed_callback {
        queue.pop_front();
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

        subprocess.state_mut().has_processed_callback = true;
    } else {
        unsafe {
            *has_callback = false;
        }
        subprocess.state_mut().has_processed_callback = false;
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
