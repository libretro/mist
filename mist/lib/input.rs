use std::{
    ffi::CStr,
    os::raw::{c_char, c_int, c_ushort},
};

use crate::{
    consts::*,
    result::{MistResult, Success},
    types::*,
};

/// Makes the input controller use the action set
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_activate_action_set(
    input_handle: MistInputHandle,
    action_set_handle: MistInputActionSetHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .activate_action_set(input_handle, action_set_handle));

    Success
}

/// Makes the input controller use the action set layer
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_activate_action_set_layer(
    input_handle: MistInputHandle,
    action_set_layer_handle: MistInputActionSetHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .activate_action_set_layer(input_handle, action_set_layer_handle));

    Success
}

/// Deactivates the input layer on the controller
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_deactivate_action_set_layer(
    input_handle: MistInputHandle,
    action_set_layer_handle: MistInputActionSetHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .deactivate_action_set_layer(input_handle, action_set_layer_handle));

    Success
}

/// Deactivates the input layer on the controller
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_deactivate_all_action_set_layers(
    input_handle: MistInputHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .deactivate_all_action_set_layers(input_handle));

    Success
}

/// Get action set handles to the current action set layers for controller
/// Puts the handles in the handles_out parameter which needs to be a array of length MIST_STEAM_INPUT_MAX_COUNT
/// The count will be put in handles_count
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_active_action_set_layers(
    input_handle: MistInputHandle,
    handles_out: *mut MistInputActionSetHandle,
    handles_count: *mut usize,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let handles = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_active_action_set_layers(input_handle));
    let handles_len = handles.len().min(MIST_STEAM_INPUT_MAX_COUNT);

    for i in 0..handles_len {
        let handle_out = unsafe { handles_out.add(i) };

        unsafe {
            *handle_out = handles[i];
        }
    }

    unsafe { *handles_count = handles_len };

    Success
}

/// Get the action set from name
/// The action set is put in action_set_handle
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_action_set_handle(
    action_set_name: *const c_char,
    action_set_handle: *mut MistInputActionSetHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let action_set_name = unsafe { CStr::from_ptr(action_set_name) }
        .to_string_lossy()
        .to_string();

    let handle = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_action_set_handle(action_set_name));

    unsafe {
        *action_set_handle = handle;
    }

    Success
}

// TODO: fn mist_steam_input_get_analog_action_data(input_handle: MistInputHandle, analog_action_handle: MistInputAnalogActionHandle) -> ();

/// Get the analog action handle from name
/// The action handle is put in analog_action_handle
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_analog_action_handle(
    action_name: *const c_char,
    analog_action_handle: *mut MistInputAnalogActionHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let action_name = unsafe { CStr::from_ptr(action_name) }
        .to_string_lossy()
        .to_string();

    let handle = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_analog_action_handle(action_name));

    unsafe {
        *analog_action_handle = handle;
    }

    Success
}

// TODO: fn mist_steam_input_get_analog_action_origins(input_handle: MistInputHandle, action_set_handle: MistInputActionSetHandle, analog_action_handle: MistInputAnalogActionHandle) -> AnalogOrigins;
/// Get all the origins for a digital action
/// Puts the origins in the origins_out parameter which needs to be a array of length MIST_STEAM_INPUT_MAX_ORIGINS
/// The count will be put in origins_count
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_analog_action_origins(
    input_handle: MistInputHandle,
    action_set_handle: MistInputActionSetHandle,
    analog_action_handle: MistInputAnalogActionHandle,
    origins_out: *mut MistInputHandle,
    origins_count: *mut usize,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let origins = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_analog_action_origins(input_handle, action_set_handle, analog_action_handle));
    let origins_len = origins.len().min(MIST_STEAM_INPUT_MAX_ORIGINS);

    for i in 0..origins_len {
        let origin_out = unsafe { origins_out.add(i) };

        unsafe {
            *origin_out = origins[i] as _;
        }
    }

    unsafe { *origins_count = origins_len };

    Success
}

/// Get the input handles for all controllers
/// Puts the handles in the handles_out parameter which needs to be a array of length MIST_STEAM_INPUT_MAX_COUNT
/// The count will be put in handles_count
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_connected_controllers(
    handles_out: *mut MistInputHandle,
    handles_count: *mut usize,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let handles = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_connected_controllers());
    let handles_len = handles.len().min(MIST_STEAM_INPUT_MAX_COUNT);

    for i in 0..handles_len {
        let handle_out = unsafe { handles_out.add(i) };

        unsafe {
            *handle_out = handles[i];
        }
    }

    unsafe { *handles_count = handles_len };

    Success
}

/// Get the input handle for a gamepad at index
/// Puts the input handle into input_handle
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_controller_for_gamepad_index(
    index: c_int,
    input_handle: *mut MistInputHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let handle = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_controller_for_gamepad_index(index));

    unsafe { *input_handle = handle };

    Success
}

/// Get the input handle for a gamepad at index
/// Puts the input handle into input_handle
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_current_action_set(
    input_handle: MistInputHandle,
    input_action_set_handle: *mut MistInputActionSetHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let handle = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_current_action_set(input_handle));

    unsafe { *input_action_set_handle = handle };

    Success
}

// TODO: fn mist_steam_input_get_digital_action_data(input_handle: MistInputHandle, digital_action_handle: MistInputDigitalActionHandle) -> ();

/// Get digital action handle from name
/// The action handke is put in input_digital_action_handle
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_digital_action_handle(
    action_name: *const c_char,
    input_digital_action_handle: *mut MistInputDigitalActionHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let action_name = unsafe { CStr::from_ptr(action_name) }
        .to_string_lossy()
        .to_string();

    let handle = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_digital_action_handle(action_name));

    unsafe {
        *input_digital_action_handle = handle;
    }

    Success
}

/// Get all the origins for a digital action
/// Puts the origins in the origins_out parameter which needs to be a array of length MIST_STEAM_INPUT_MAX_ORIGINS
/// The count will be put in origins_count
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_digital_action_origins(
    input_handle: MistInputHandle,
    action_set_handle: MistInputActionSetHandle,
    digital_action_handle: MistInputDigitalActionHandle,
    origins_out: *mut MistInputHandle,
    origins_count: *mut usize,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let origins = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_digital_action_origins(input_handle, action_set_handle, digital_action_handle));
    let origins_len = origins.len().min(MIST_STEAM_INPUT_MAX_ORIGINS);

    for i in 0..origins_len {
        let origin_out = unsafe { origins_out.add(i) };

        unsafe {
            *origin_out = origins[i] as _;
        }
    }

    unsafe { *origins_count = origins_len };

    Success
}

/// Get the gamepad index from an input handle.
/// Puts the gamepad index into index
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_gamepad_index_for_controller(
    input_handle: MistInputHandle,
    index: *mut c_int,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let handle = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_gamepad_index_for_controller(input_handle));

    unsafe { *index = handle };

    Success
}

/// Get the gamepad index from an input handle.
/// Puts the gamepad index into index
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_glyph_png_for_action_origin(
    origin: MistInputActionOrigin,
    size: MistSteamInputGlyphSize,
    flags: MistSteamInputGlyphStyle,
    path: *mut *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let path_cstr = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_glyph_png_for_action_origin(origin, size, flags));

    let path_cstr_stored = subprocess
        .state_mut()
        .glpyh_png
        .entry((origin, size, flags))
        .or_insert_with(|| path_cstr);

    unsafe { *path = path_cstr_stored.as_ptr() };

    Success
}

/// Get the gamepad index from an input handle.
/// Puts the gamepad index into index
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_glyph_svg_for_action_origin(
    origin: MistInputActionOrigin,
    flags: MistSteamInputGlyphStyle,
    path: *mut *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let path_cstr = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_glyph_svg_for_action_origin(origin, flags));

    let path_cstr_stored = subprocess
        .state_mut()
        .glpyh_svg
        .entry((origin, flags))
        .or_insert_with(|| path_cstr);

    unsafe { *path = path_cstr_stored.as_ptr() };

    Success
}

/// Get input type for controller
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_input_type_for_handle(
    input_handle: MistInputHandle,
    input_type: *mut MistSteamInputType,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let input_ty = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_input_type_for_handle(input_handle));

    unsafe { *input_type = input_ty };

    Success
}

// TODO: fn mist_steam_input_get_motion_data(input_handle: MistInputHandle) -> InputMotionData;

/// Get the string from origin
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_get_string_for_action_origin(
    origin: MistInputActionOrigin,
    string: *mut *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let origin_string_cstr = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .get_string_for_action_origin(origin));

    let origin_string_cstr_stored = subprocess
        .state_mut()
        .origin_strings
        .entry(origin)
        .or_insert_with(|| origin_string_cstr);

    unsafe { *string = origin_string_cstr_stored.as_ptr() };

    Success
}

/// Inits steam input
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_init() -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().steam_input().init());

    Success
}

/// Sets the led color of a controller
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_set_led_color(
    input_handle: MistInputHandle,
    color_r: u8,
    color_g: u8,
    color_b: u8,
    flags: MistSteamControllerLEDFlag,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().steam_input().set_led_color(
        input_handle,
        color_r,
        color_g,
        color_b,
        flags
    ));

    Success
}

/// Shows the steam input binding menu for a controller
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_show_binding_panel(
    input_handle: MistInputHandle,
    overlay_shown: *mut bool,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let shown = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .show_binding_panel(input_handle));

    unsafe { *overlay_shown = shown };

    Success
}

/// Shuts down steam input
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_shutdown() -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().steam_input().shutdown());

    Success
}

/// Stops the virtual analog momentum
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_stop_analog_action_momentum(
    input_handle: MistInputHandle,
    action: MistInputAnalogActionHandle,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .stop_analog_action_momentum(input_handle, action));

    Success
}

/// Trigger vibration
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_trigger_vibration(
    input_handle: MistInputHandle,
    left_speed: c_ushort,
    right_speed: c_ushort,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().steam_input().trigger_vibration(
        input_handle,
        left_speed,
        right_speed
    ));

    Success
}

/// Trigger vibration extended
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_trigger_vibration_extended(
    input_handle: MistInputHandle,
    left_speed: c_ushort,
    right_speed: c_ushort,
    left_trigger_speed: c_ushort,
    right_trigger_speed: c_ushort,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .trigger_vibration_extended(
            input_handle,
            left_speed,
            right_speed,
            left_trigger_speed,
            right_trigger_speed
        ));

    Success
}

/// Trigger haptic event
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_trigger_simple_haptic_event(
    input_handle: MistInputHandle,
    haptic_location: MistControllerHapticLocation,
    intensity: u8,
    gain_db: c_char,
    other_intensity: u8,
    other_gain_db: c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .trigger_simple_haptic_event(
            input_handle,
            haptic_location,
            intensity,
            gain_db,
            other_intensity,
            other_gain_db
        ));

    Success
}

/// Translate origin to other input type origin
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_input_translate_action_origin(
    destination_input_type: MistSteamInputType,
    source_origin: MistInputActionOrigin,
    translated_origin: *mut MistInputActionOrigin,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let translated = unwrap_client_result!(subprocess
        .client()
        .steam_input()
        .translate_action_origin(destination_input_type, source_origin));

    unsafe { *translated_origin = translated };

    Success
}
