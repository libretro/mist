use crate::{
    result::{MistResult, Success},
    types::AppId,
};

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
