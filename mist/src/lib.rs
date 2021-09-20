use std::os::raw::c_char;

/// Init mist, this is a no-op if it is already running, returns true on error
#[no_mangle]
pub extern "C" fn mist_init() -> bool {
    false
}

/// Returns the latest error
#[no_mangle]
pub extern "C" fn mist_geterror() -> *const c_char {
    let null: &[c_char] = &[0];

    null.as_ptr()
}

/// Polls the subprocess, returns true on error
#[no_mangle]
pub extern "C" fn mist_poll() -> bool {
    false
}

/// Deinits the runtime, returns true on error
#[no_mangle]
pub extern "C" fn mist_deinit() -> bool {
    false
}
