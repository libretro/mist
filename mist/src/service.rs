use crate::types::*;

// Service calls for the subprocess
mist_service!(
    // ISteamApps
    Apps {
        fn get_dlc_data_by_index(dlc: i32) -> Option<DlcData>;
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
    Friends {
        fn clear_rich_presence();
        fn set_rich_presence(key: String, value: Option<String>) -> bool;
    }

    // ISteamUtils
    Utils {
        fn get_appid() -> AppId;
        fn is_steam_running_on_steam_deck() -> bool;
    }

    // Internal
    Internal {
        fn exit();
    }
);
