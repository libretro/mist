use std::os::raw::c_int;

use crate::types::*;

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
