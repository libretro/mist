use std::{ffi::CStr, os::raw::c_char};

use crate::result::{MistResult, Success};

/// Clears the rich presence key/value store
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_friends_clear_rich_presence() -> MistResult {
    let mut subprocess = get_subprocess!();
    unwrap_client_result!(subprocess.client().steam_friends().clear_rich_presence());

    Success
}

/// Sets the rich presence key/value
/// Value can be NULL to clear the key
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_friends_set_rich_presence(
    key: *const c_char,
    value: *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let key = unsafe { CStr::from_ptr(key) }.to_string_lossy().to_string();
    let value = if value.is_null() {
        None
    } else {
        Some(
            unsafe { CStr::from_ptr(value) }
                .to_string_lossy()
                .to_string(),
        )
    };

    unwrap_client_result!(subprocess
        .client()
        .steam_friends()
        .set_rich_presence(key, value));

    Success
}
