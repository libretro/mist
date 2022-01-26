use std::{ffi::CStr, os::raw::c_char};

/// Clears the rich presence key/value store
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_friends_clear_rich_presence() -> bool {
    let subprocess = get_subprocess!();
    subprocess.client().friends().clear_rich_presence();

    true
}

/// Sets the rich presence key/value
/// Value can be NULL to clear the key
/// Returns false on error
#[no_mangle]
pub extern "C" fn mist_friends_set_rich_presence(key: *const c_char, value: *const c_char) -> bool {
    let subprocess = get_subprocess!();

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

    unwrap_client_result!(subprocess.client().friends().set_rich_presence(key, value))
}
