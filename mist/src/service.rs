use std::{
    ffi::CString,
    os::raw::{c_char, c_int, c_ushort},
};

use crate::types::*;

const DEFAULT_TIMEOUT: u64 = 100;

// Service calls for the subprocess
mist_service!(
    // ISteamApps
    SteamApps {
        fn get_dlc_data_by_index(dlc: i32) -> DlcData;
        fn is_app_installed(app_id: AppId) -> bool;
        fn is_cybercafe() -> bool;
        fn is_dlc_installed(app_id: AppId) -> bool;
        fn is_low_violence() -> bool;
        fn is_subscribed() -> bool;
        fn is_subscribed_app(app_id: AppId) -> bool;
        fn is_subscribed_from_family_sharing() -> bool;
        fn is_subscribed_from_free_weekend() -> bool;
        fn is_vac_banned() -> bool;
        fn get_app_build_id() -> BuildId;
        fn get_app_install_dir(app_id: AppId) -> Option<String>;
        fn get_app_owner() -> SteamId;
        fn get_available_game_languages() -> String;
        fn get_current_beta_name() -> Option<String>;
        fn get_current_game_language() -> String;
        fn get_dlc_count() -> i32;
        fn get_dlc_download_progress(app_id: AppId) -> Option<(u64, u64)>;
        fn get_earliest_purchase_unix_time(app_id: AppId) -> u32;
        //fn get_file_details(file_name: String) -> (); TODO (async)
        fn get_installed_depots(app_id: AppId) -> Vec<DepotId>;
        fn get_launch_command_line() -> String;
        fn get_launch_query_param(key: String) -> Option<String>;
        fn install_dlc(app_id: AppId);
        fn mark_content_corrupt(missing_files_only: bool) -> bool;
        fn uninstall_dlc(app_id: AppId);
    }

    // ISteamFriends
    SteamFriends {
        fn clear_rich_presence();
        fn set_rich_presence(key: String, value: Option<String>);
    }

    // ISteamInput
    SteamInput {
        fn activate_action_set(input_handle: MistInputHandle, action_set_handle: MistInputActionSetHandle);
        fn activate_action_set_layer(input_handle: MistInputHandle, action_set_layer_handle: MistInputActionSetHandle);
        fn deactivate_action_set_layer(input_handle: MistInputHandle, action_set_layer_handle: MistInputActionSetHandle);
        fn deactivate_all_action_set_layers(input_handle: MistInputHandle);
        fn get_active_action_set_layers(input_handle: MistInputHandle) -> Vec<MistInputActionSetHandle>;
        fn get_action_set_handle(action_set_name: String) -> MistInputActionSetHandle;
        // fn get_analog_action_data(input_handle: MistInputHandle, analog_action_handle: MistInputAnalogActionHandle) -> (); - Stored in shared memory.
        fn get_analog_action_handle(name: String) -> MistInputAnalogActionHandle;
        fn get_analog_action_origins(input_handle: MistInputHandle, action_set_handle: MistInputActionSetHandle, analog_action_handle: MistInputAnalogActionHandle) -> Vec<MistInputActionOrigin>;
        fn get_connected_controllers() -> Vec<MistInputHandle>;
        fn get_controller_for_gamepad_index(index: c_int) -> MistInputHandle;
        fn get_current_action_set(input_handle: MistInputHandle) -> MistInputActionSetHandle;
        // fn get_digital_action_data(input_handle: MistInputHandle, digital_action_handle: MistInputDigitalActionHandle) -> -> (); - Stored in shared memory.
        fn get_digital_action_handle(name: String) -> MistInputDigitalActionHandle;
        fn get_digital_action_origins(input_handle: MistInputHandle, action_set_handle: MistInputActionSetHandle, digital_action_handle: MistInputDigitalActionHandle) -> Vec<MistInputActionOrigin>;
        fn get_gamepad_index_for_controller(controller_handle: MistInputHandle) -> c_int;
        fn get_glyph_png_for_action_origin(origin: MistInputActionOrigin, size: MistSteamInputGlyphSize,  flags: MistSteamInputGlyphStyle) -> CString;
        fn get_glyph_svg_for_action_origin(origin: MistInputActionOrigin, flags: MistSteamInputGlyphStyle) -> CString;
        fn get_input_type_for_handle(input_handle: MistInputHandle) -> MistSteamInputType;
        // fn get_motion_data(input_handle: MistInputHandle) -> (); - Stored in shared memory.
        fn get_string_for_action_origin(origin: MistInputActionOrigin) -> CString;
        #[timeout(1_000)]
        fn init() -> (String, bool);
        #[timeout(10_000)]
        fn set_input_action_manifest_file_path(path: CString) -> bool;
        fn set_led_color(input_handle: MistInputHandle, color_r: u8, color_g: u8, color_b: u8, flags: MistSteamControllerLEDFlag);
        // fn show_analog_action_origins... Deprecated so not implemented
        fn show_binding_panel(input_handle: MistInputHandle) -> bool;
        // fn show_digital_action_origins... Deprecated so not implemented
        fn shutdown() -> bool;
        fn stop_analog_action_momentum(input_handle: MistInputHandle, action: MistInputAnalogActionHandle);
        fn trigger_vibration(input_handle: MistInputHandle, left_speed: c_ushort, right_speed: c_ushort);
        fn trigger_vibration_extended(input_handle: MistInputHandle, left_speed: c_ushort, right_speed: c_ushort, left_trigger_speed: c_ushort, right_trigger_speed: c_ushort);
        fn trigger_simple_haptic_event(input_handle: MistInputHandle, haptic_location: MistControllerHapticLocation, intensity: u8, gain_db: c_char, other_intensity: u8,other_gain_db: c_char);
        fn translate_action_origin(destination_input_type: MistSteamInputType, source_origin: MistInputActionOrigin) -> MistInputActionOrigin;
}

    // ISteamRemoteStorage
    SteamRemoteStorage {
        fn begin_file_write_batch();
        fn end_file_write_batch();
    }

    // ISteamUtils
    SteamUtils {
        fn get_appid() -> AppId;
        fn get_current_battery_power() -> u8;
        fn get_entered_gamepad_text_input() -> Option<String>;
        fn is_overlay_enabled() -> bool;
        fn is_steam_in_big_picture_mode() -> bool;
        fn is_steam_running_in_vr() -> bool;
        fn is_vr_headset_streaming_enabled() -> bool;
        fn is_steam_running_on_steam_deck() -> bool;
        fn set_vr_headset_streaming_enabled(enabled: bool);
        fn show_gamepad_text_input(
            input_mode: MistGamepadTextInputMode,
            line_input_mode: MistGamepadTextInputLineMode,
            description: String,
            char_max: u32,
            existing_text: String
        ) -> bool;
        fn show_floating_gamepad_text_input(
            keyboard_mode: MistFloatingGamepadTextInputMode,
            text_field_x_position: c_int,
            text_field_y_position: c_int,
            text_field_width: c_int,
            text_field_height: c_int
        ) -> bool;
        fn set_game_launcher_mode(launcher_mode: bool);
        fn start_vr_dashboard();
    }

    // Internal
    Internal {
        fn exit();
    }
);
