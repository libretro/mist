use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

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
                return true;
            }
        }
    };
}

pub fn mist_set_error(err: &str) {
    unsafe { LAST_ERROR = Some(CString::new(err).unwrap()) };
}

/// Init mist, this is throwns an error if it was already initialised, returns true on error
#[no_mangle]
pub extern "C" fn mist_subprocess_init() -> bool {
    let result = std::panic::catch_unwind(|| lib_subprocess::mist_init_subprocess());

    match result {
        Ok(err) => err,
        Err(_) => {
            mist_set_error("Internal panic during initialization");
            return true;
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

/// Polls the subprocess, returns true on error
#[no_mangle]
pub extern "C" fn mist_poll() -> bool {
    let _subprocess = get_subprocess!();
    false
}

/// Clears the rich presence key/value store
#[no_mangle]
pub extern "C" fn mist_friends_clear_rich_presence() -> bool {
    let subprocess = get_subprocess!();
    subprocess.client().clear_rich_presence();

    false
}

/// Sets the rich presence key/value
/// Value can be NULL to clear the key
#[no_mangle]
pub extern "C" fn mist_friends_set_rich_presence(key: *const i8, value: *const i8) -> bool {
    let subprocess = get_subprocess!();

    let key = unsafe { CStr::from_ptr(key) }.to_string_lossy().to_string();
    let value = if value == std::ptr::null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(value) }
                .to_string_lossy()
                .to_string(),
        )
    };

    // set rich presence returns true on success, so invert it
    !unwrap_client_result!(subprocess.client().set_rich_presence(key, value))
}

/// Returns the appid of the running application
#[no_mangle]
pub extern "C" fn mist_utils_get_appid(app_id: *mut u32) -> bool {
    let subprocess = get_subprocess!();

    let id = unwrap_client_result!(subprocess.client().get_appid());

    unsafe {
        *app_id = id;
    }

    false
}

/// Deinits the mist subprocess, returns true on error
#[no_mangle]
pub extern "C" fn mist_subprocess_deinit() -> bool {
    lib_subprocess::mist_deinit_subprocess()
}
