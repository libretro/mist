use std::{
    ffi::CStr,
    os::raw::{c_char, c_int},
};

use crate::{
    result::{Error, MistResult, SteamUtilsError, Success},
    types::{
        AppId, MistFloatingGamepadTextInputMode, MistGamepadTextInputLineMode,
        MistGamepadTextInputMode,
    },
};

pub static mut ENTERED_GAMEPAD_TEXT: *mut String = std::ptr::null_mut();

/// Returns the appid of the running application in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_get_appid(app_id: *mut AppId) -> MistResult {
    let subprocess = get_subprocess!();

    let id = unwrap_client_result!(subprocess.client().steam_utils().get_appid());

    unsafe {
        *app_id = id;
    }

    Success
}

/// Returns the battery percentage in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_get_current_battery_power(battery_power: *mut u8) -> MistResult {
    let subprocess = get_subprocess!();

    let power = unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .get_current_battery_power());

    unsafe {
        *battery_power = power;
    }

    Success
}

/// Copies the entered gamepad text to `text` buffer of `text_size`
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_get_entered_gamepad_text_input(
    text: *mut c_char,
    text_size: u32,
) -> MistResult {
    let subprocess = get_subprocess!();
    let text_size = text_size as usize;

    if unsafe { ENTERED_GAMEPAD_TEXT.is_null() } {
        let entered = unwrap_client_result!(subprocess
            .client()
            .steam_utils()
            .get_entered_gamepad_text_input());

        if let Some(entered) = entered {
            unsafe {
                std::ptr::copy_nonoverlapping(
                    entered.as_ptr() as *mut i8,
                    text,
                    entered.len().min(text_size),
                );
            }
        } else {
            return Error::SteamUtils(SteamUtilsError::NoGamepadTextEntered).into();
        }
    } else {
        {
            let entered = unsafe { &*ENTERED_GAMEPAD_TEXT };
            unsafe {
                std::ptr::copy_nonoverlapping(
                    entered.as_ptr() as *mut i8,
                    text,
                    entered.len().min(text_size),
                );
            }
        }

        drop(unsafe { Box::from_raw(ENTERED_GAMEPAD_TEXT) });
        unsafe { ENTERED_GAMEPAD_TEXT = std::ptr::null_mut() }
    }

    Success
}

/// Sets the length out ptr to the length of the entered gamepad text
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_get_entered_gamepad_text_length(length: *mut u32) -> MistResult {
    let subprocess = get_subprocess!();

    let entered = unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .get_entered_gamepad_text_input());

    if let Some(entered) = entered {
        if !unsafe { ENTERED_GAMEPAD_TEXT.is_null() } {
            drop(unsafe { Box::from_raw(ENTERED_GAMEPAD_TEXT) });
            unsafe { ENTERED_GAMEPAD_TEXT = std::ptr::null_mut() }
        }

        unsafe {
            *length = entered.len() as u32;
            ENTERED_GAMEPAD_TEXT = Box::into_raw(Box::new(entered));
        }
    } else {
        return Error::SteamUtils(SteamUtilsError::NoGamepadTextEntered).into();
    }

    Success
}

/// Return if the Steam overlay is enabled in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_is_overlay_enabled(overlay_enabled: *mut bool) -> MistResult {
    let subprocess = get_subprocess!();

    let enabled = unwrap_client_result!(subprocess.client().steam_utils().is_overlay_enabled());

    unsafe {
        *overlay_enabled = enabled;
    }

    Success
}

/// Return if Steam is running in Big Picture mode in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_is_steam_in_big_picture_mode(
    in_big_picture: *mut bool,
) -> MistResult {
    let subprocess = get_subprocess!();

    let big_picture = unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .is_steam_in_big_picture_mode());

    unsafe {
        *in_big_picture = big_picture;
    }

    Success
}

/// Return if Steam is running in VR mode in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_is_steam_running_in_vr(running_in_vr: *mut bool) -> MistResult {
    let subprocess = get_subprocess!();

    let in_vr = unwrap_client_result!(subprocess.client().steam_utils().is_steam_running_in_vr());

    unsafe {
        *running_in_vr = in_vr;
    }

    Success
}

/// Return if VR view streaming via Steam Remote Play is enabled in the out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_is_vr_headset_streaming_enabled(
    vr_streaming_enabled: *mut bool,
) -> MistResult {
    let subprocess = get_subprocess!();

    let enabled = unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .is_vr_headset_streaming_enabled());

    unsafe {
        *vr_streaming_enabled = enabled;
    }

    Success
}

/// Return if steam is running on a steam deck in the out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_is_steam_running_on_steam_deck(
    on_deck: *mut bool,
) -> MistResult {
    let subprocess = get_subprocess!();

    let result = unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .is_steam_running_on_steam_deck());

    unsafe {
        *on_deck = result;
    }

    Success
}

/// Set if Steam Remote Play should be avaliable for HMD content
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_set_vr_headset_streaming_enabled(enabled: bool) -> MistResult {
    let subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .set_vr_headset_streaming_enabled(enabled));

    Success
}

/// Showing a floating keyboard over the game and sends input directly to it
/// Returns if shown in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_show_gamepad_text_input(
    input_mode: MistGamepadTextInputMode,
    line_input_mode: MistGamepadTextInputLineMode,
    description: *const c_char,
    char_max: u32,
    existing_text: *const c_char,
    shown: *mut bool,
) -> MistResult {
    let subprocess = get_subprocess!();

    let description = unsafe { CStr::from_ptr(description) }
        .to_string_lossy()
        .to_string();
    let existing_text = unsafe { CStr::from_ptr(existing_text) }
        .to_string_lossy()
        .to_string();

    let did_show =
        unwrap_client_result!(subprocess.client().steam_utils().show_gamepad_text_input(
            input_mode,
            line_input_mode,
            description,
            char_max,
            existing_text
        ));

    unsafe {
        *shown = did_show;
    }

    Success
}

/// Showing a floating keyboard over the game and sends input directly to it
/// Returns if shown in out ptr
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_show_floating_gamepad_text_input(
    keyboard_mode: MistFloatingGamepadTextInputMode,
    text_field_x_position: c_int,
    text_field_y_position: c_int,
    text_field_width: c_int,
    text_field_height: c_int,
    shown: *mut bool,
) -> MistResult {
    let subprocess = get_subprocess!();

    let did_show = unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .show_floating_gamepad_text_input(
            keyboard_mode,
            text_field_x_position,
            text_field_y_position,
            text_field_width,
            text_field_height
        ));

    unsafe {
        *shown = did_show;
    }

    Success
}

/// Make Steam translate controller input into mouse/kb for UI that does not support controllers
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_set_game_launcher_mode(launcher_mode: bool) -> MistResult {
    let subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_utils()
        .set_game_launcher_mode(launcher_mode));

    Success
}

/// Open the VR dashboard
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_start_vr_dashboard() -> MistResult {
    let subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().steam_utils().start_vr_dashboard());

    Success
}
