use std::{ffi::CString, os::raw::c_char};

#[macro_use]
mod codegen;
mod consts;
mod service;
#[macro_use]
mod lib_subprocess;
mod types;

static mut LAST_ERROR: Option<CString> = None;

macro_rules! unwrap_client_result {
    ($res:expr) => {
        match $res {
            Some(res) => res,
            None => {
                return false;
            }
        }
    };
}

pub fn mist_set_error(err: &str) {
    unsafe { LAST_ERROR = Some(CString::new(err).unwrap()) };
}

/// Init mist, this is throwns an error if it was already initialised, returns false on error
#[no_mangle]
pub extern "C" fn mist_subprocess_init() -> bool {
    let result = std::panic::catch_unwind(|| lib_subprocess::mist_init_subprocess());

    match result {
        Ok(err) => err,
        Err(_) => {
            mist_set_error("Internal panic during initialization");
            return false;
        }
    }
}

/// Returns the latest error
#[no_mangle]
pub extern "C" fn mist_geterror() -> *const c_char {
    // Check if we have an error error, otherwise return an pointer to a single null character
    if let Some(err) = unsafe { &LAST_ERROR } {
        err.as_ptr()
    } else {
        let null: &[c_char] = &[0];

        null.as_ptr()
    }
}

/// Polls the subprocess, returns false on error
#[no_mangle]
pub extern "C" fn mist_poll() -> bool {
    let _subprocess = get_subprocess!();
    true
}

#[path = "../lib/apps.rs"]
mod apps;
#[path = "../lib/friends.rs"]
mod friends;
#[path = "../lib/utils.rs"]
mod utils;

/// Deinits the mist subprocess, returns false on error
#[no_mangle]
pub extern "C" fn mist_subprocess_deinit() -> bool {
    lib_subprocess::mist_deinit_subprocess()
}
