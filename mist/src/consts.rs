// Used to throw an error when running the subprocess directly
pub const PROCESS_INIT_SECRET: &str = "youarenotsupposedtorunmistdirectly";

/// cbindgen:ignore
pub const MIST_INPUT_STATE_BUFFER_SIZE: u8 = 3;

pub const MIST_STEAM_INPUT_MAX_COUNT: usize = 16;
pub const MIST_STEAM_INPUT_MAX_ANALOG_ACTIONS: usize = 24;
pub const MIST_STEAM_INPUT_MAX_DIGITAL_ACTIONS: usize = 256;
pub const MIST_STEAM_INPUT_MAX_ORIGINS: usize = 8;
pub const MIST_STEAM_INPUT_MAX_ACTIVE_LAYERS: usize = 16;

pub const MIST_STEAM_INPUT_HANDLE_ALL_CONTROLLERS: u64 = u64::max_value();
pub const MIST_MAX_GAMEPADS: usize = 16;
/// sizeof(ScePadTriggerEffectParam) from the Steamworks SDK isteamdualsense.h
pub const MIST_STEAM_INPUT_DUALSENSE_TRIGGER_EFFECT_PARAM_SIZE: usize = 120;
