// Service calls for the subprocess
mist_service!(
    // Friends
    fn clear_rich_presence();
    fn set_rich_presence(key: String, value: Option<String>) -> bool;
    // Utils
    fn get_appid() -> u32;
    fn is_steam_running_on_steam_deck() -> bool;
);
