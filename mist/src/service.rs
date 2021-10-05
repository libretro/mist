use crate::types::*;

// Service calls for the subprocess
mist_service!(
    // ISteamApps
    /*Apps {
        fn apps_get_dlc_data_by_index(dlc: i32, app_id: AppId) -> ();
        fn apps_is_app_installed(app_id: AppId) -> bool;
        fn apps_is_cybercafe() -> bool;
        fn apps_is_dlc_installed(app_id: AppId) -> bool;
        fn apps_is_low_violence() -> bool;
        fn apps_is_subscribed() -> bool;
        fn apps_is_subscribed_app(app_id: AppId) -> bool;
        fn apps_is_subscribed_from_family_sharing() -> bool;
        fn apps_is_subscribed_from_free_weekend() -> bool;
        fn apps_is_vac_banned() -> bool;
        fn apps_get_app_build_id() -> BuildId;
        fn apps_get_app_install_dir(app_id: AppId) -> String;
        //fn apps_get_app_owner() -> SteamID;
        fn apps_get_available_game_languages() -> String;
        fn apps_get_current_beta_beta_name() -> String;
        fn apps_get_current_game_language() -> String;
        fn apps_get_dlc_count() -> i32;
        fn apps_get_dlc_download_progress(app_id: AppId) -> Option<(u64, u64)>;
        fn apps_get_earliest_purchase_unix_time(app_id: AppId) -> u32;
        // fn apps_get_file_details(file_name: String) -> FileDetails;
        fn apps_get_installed_depots(app_id: AppId) -> Vec<DepotId>;
        fn apps_get_launch_command_line() -> String;
        fn apps_get_launch_query_param(key: String) -> String;
        fn apps_install_dlc(app_id: AppId);
        fn apps_mark_content_corrupt(missing_files_only: bool);
        fn apps_uninstall_dlc(app_id: AppId);
    }*/

    // ISteamFriends
    Friends {
        fn friends_clear_rich_presence();
        fn friends_set_rich_presence(key: String, value: Option<String>) -> bool;
    }

    // ISteamUtils
    Utils {
        fn utils_get_appid() -> AppId;
        fn utils_is_steam_running_on_steam_deck() -> bool;
    }

    // Internal
    Internal {
        fn exit();
    }
);
