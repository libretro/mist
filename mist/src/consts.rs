// Used to throw an error when running the subprocess directly
pub const PROCESS_INIT_SECRET: &str = "youarenotsupposedtorunmistdirectly";

pub const MIST_STEAM_INPUT_MAX_COUNT: usize = 16;
pub const MIST_STEAM_INPUT_MAX_ANALOG_ACTIONS: usize = 16;
pub const MIST_STEAM_INPUT_MAX_DIGITAL_ACTIONS: usize = 128;
pub const MIST_STEAM_INPUT_MAX_ORIGINS: usize = 8;
pub const MIST_STEAM_INPUT_MAX_ACTIVE_LAYERS: usize = 16;

pub const MIST_STEAM_INPUT_HANDLE_ALL_CONTROLLERS: u64 = u64::max_value();
