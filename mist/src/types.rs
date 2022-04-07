use serde_derive::{Deserialize, Serialize};
use std::os::raw::c_float;

use crate::consts::*;

pub type AppId = u32;
pub type BuildId = i32;
pub type DepotId = u32;
pub type SteamId = u64;
pub type SteamUser = i32;

// SteamInput types
pub type MistInputHandle = u64;
pub type MistInputActionSetHandle = u64;
pub type MistInputAnalogActionHandle = u64;
pub type MistInputDigitalActionHandle = u64;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DlcData {
    pub app_id: AppId,
    pub avaliable: bool,
    pub name: String,
}

// Steam Input
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(u32)]
pub enum MistInputActionOrigin {
    None = 0,
    A = 1,
    B = 2,
    X = 3,
    Y = 4,
    LeftBumper = 5,
    RightBumper = 6,
    LeftGrip = 7,
    RightGrip = 8,
    Start = 9,
    Back = 10,
    LeftPad_Touch = 11,
    LeftPad_Swipe = 12,
    LeftPad_Click = 13,
    LeftPad_DPadNorth = 14,
    LeftPad_DPadSouth = 15,
    LeftPad_DPadWest = 16,
    LeftPad_DPadEast = 17,
    RightPad_Touch = 18,
    RightPad_Swipe = 19,
    RightPad_Click = 20,
    RightPad_DPadNorth = 21,
    RightPad_DPadSouth = 22,
    RightPad_DPadWest = 23,
    RightPad_DPadEast = 24,
    LeftTrigger_Pull = 25,
    LeftTrigger_Click = 26,
    RightTrigger_Pull = 27,
    RightTrigger_Click = 28,
    LeftStick_Move = 29,
    LeftStick_Click = 30,
    LeftStick_DPadNorth = 31,
    LeftStick_DPadSouth = 32,
    LeftStick_DPadWest = 33,
    LeftStick_DPadEast = 34,
    Gyro_Move = 35,
    Gyro_Pitch = 36,
    Gyro_Yaw = 37,
    Gyro_Roll = 38,
    SteamController_Reserved0 = 39,
    SteamController_Reserved1 = 40,
    SteamController_Reserved2 = 41,
    SteamController_Reserved3 = 42,
    SteamController_Reserved4 = 43,
    SteamController_Reserved5 = 44,
    SteamController_Reserved6 = 45,
    SteamController_Reserved7 = 46,
    SteamController_Reserved8 = 47,
    SteamController_Reserved9 = 48,
    SteamController_Reserved10 = 49,
    PS4_X = 50,
    PS4_Circle = 51,
    PS4_Triangle = 52,
    PS4_Square = 53,
    PS4_LeftBumper = 54,
    PS4_RightBumper = 55,
    PS4_Options = 56,
    PS4_Share = 57,
    PS4_LeftPad_Touch = 58,
    PS4_LeftPad_Swipe = 59,
    PS4_LeftPad_Click = 60,
    PS4_LeftPad_DPadNorth = 61,
    PS4_LeftPad_DPadSouth = 62,
    PS4_LeftPad_DPadWest = 63,
    PS4_LeftPad_DPadEast = 64,
    PS4_RightPad_Touch = 65,
    PS4_RightPad_Swipe = 66,
    PS4_RightPad_Click = 67,
    PS4_RightPad_DPadNorth = 68,
    PS4_RightPad_DPadSouth = 69,
    PS4_RightPad_DPadWest = 70,
    PS4_RightPad_DPadEast = 71,
    PS4_CenterPad_Touch = 72,
    PS4_CenterPad_Swipe = 73,
    PS4_CenterPad_Click = 74,
    PS4_CenterPad_DPadNorth = 75,
    PS4_CenterPad_DPadSouth = 76,
    PS4_CenterPad_DPadWest = 77,
    PS4_CenterPad_DPadEast = 78,
    PS4_LeftTrigger_Pull = 79,
    PS4_LeftTrigger_Click = 80,
    PS4_RightTrigger_Pull = 81,
    PS4_RightTrigger_Click = 82,
    PS4_LeftStick_Move = 83,
    PS4_LeftStick_Click = 84,
    PS4_LeftStick_DPadNorth = 85,
    PS4_LeftStick_DPadSouth = 86,
    PS4_LeftStick_DPadWest = 87,
    PS4_LeftStick_DPadEast = 88,
    PS4_RightStick_Move = 89,
    PS4_RightStick_Click = 90,
    PS4_RightStick_DPadNorth = 91,
    PS4_RightStick_DPadSouth = 92,
    PS4_RightStick_DPadWest = 93,
    PS4_RightStick_DPadEast = 94,
    PS4_DPad_North = 95,
    PS4_DPad_South = 96,
    PS4_DPad_West = 97,
    PS4_DPad_East = 98,
    PS4_Gyro_Move = 99,
    PS4_Gyro_Pitch = 100,
    PS4_Gyro_Yaw = 101,
    PS4_Gyro_Roll = 102,
    PS4_Reserved0 = 103,
    PS4_Reserved1 = 104,
    PS4_Reserved2 = 105,
    PS4_Reserved3 = 106,
    PS4_Reserved4 = 107,
    PS4_Reserved5 = 108,
    PS4_Reserved6 = 109,
    PS4_Reserved7 = 110,
    PS4_Reserved8 = 111,
    PS4_Reserved9 = 112,
    PS4_Reserved10 = 113,
    XBoxOne_A = 114,
    XBoxOne_B = 115,
    XBoxOne_X = 116,
    XBoxOne_Y = 117,
    XBoxOne_LeftBumper = 118,
    XBoxOne_RightBumper = 119,
    XBoxOne_Menu = 120,
    XBoxOne_View = 121,
    XBoxOne_LeftTrigger_Pull = 122,
    XBoxOne_LeftTrigger_Click = 123,
    XBoxOne_RightTrigger_Pull = 124,
    XBoxOne_RightTrigger_Click = 125,
    XBoxOne_LeftStick_Move = 126,
    XBoxOne_LeftStick_Click = 127,
    XBoxOne_LeftStick_DPadNorth = 128,
    XBoxOne_LeftStick_DPadSouth = 129,
    XBoxOne_LeftStick_DPadWest = 130,
    XBoxOne_LeftStick_DPadEast = 131,
    XBoxOne_RightStick_Move = 132,
    XBoxOne_RightStick_Click = 133,
    XBoxOne_RightStick_DPadNorth = 134,
    XBoxOne_RightStick_DPadSouth = 135,
    XBoxOne_RightStick_DPadWest = 136,
    XBoxOne_RightStick_DPadEast = 137,
    XBoxOne_DPad_North = 138,
    XBoxOne_DPad_South = 139,
    XBoxOne_DPad_West = 140,
    XBoxOne_DPad_East = 141,
    XBoxOne_Reserved0 = 142,
    XBoxOne_Reserved1 = 143,
    XBoxOne_Reserved2 = 144,
    XBoxOne_Reserved3 = 145,
    XBoxOne_Reserved4 = 146,
    XBoxOne_Reserved5 = 147,
    XBoxOne_Reserved6 = 148,
    XBoxOne_Reserved7 = 149,
    XBoxOne_Reserved8 = 150,
    XBoxOne_Reserved9 = 151,
    XBoxOne_Reserved10 = 152,
    XBox360_A = 153,
    XBox360_B = 154,
    XBox360_X = 155,
    XBox360_Y = 156,
    XBox360_LeftBumper = 157,
    XBox360_RightBumper = 158,
    XBox360_Start = 159,
    XBox360_Back = 160,
    XBox360_LeftTrigger_Pull = 161,
    XBox360_LeftTrigger_Click = 162,
    XBox360_RightTrigger_Pull = 163,
    XBox360_RightTrigger_Click = 164,
    XBox360_LeftStick_Move = 165,
    XBox360_LeftStick_Click = 166,
    XBox360_LeftStick_DPadNorth = 167,
    XBox360_LeftStick_DPadSouth = 168,
    XBox360_LeftStick_DPadWest = 169,
    XBox360_LeftStick_DPadEast = 170,
    XBox360_RightStick_Move = 171,
    XBox360_RightStick_Click = 172,
    XBox360_RightStick_DPadNorth = 173,
    XBox360_RightStick_DPadSouth = 174,
    XBox360_RightStick_DPadWest = 175,
    XBox360_RightStick_DPadEast = 176,
    XBox360_DPad_North = 177,
    XBox360_DPad_South = 178,
    XBox360_DPad_West = 179,
    XBox360_DPad_East = 180,
    XBox360_Reserved0 = 181,
    XBox360_Reserved1 = 182,
    XBox360_Reserved2 = 183,
    XBox360_Reserved3 = 184,
    XBox360_Reserved4 = 185,
    XBox360_Reserved5 = 186,
    XBox360_Reserved6 = 187,
    XBox360_Reserved7 = 188,
    XBox360_Reserved8 = 189,
    XBox360_Reserved9 = 190,
    XBox360_Reserved10 = 191,
    Switch_A = 192,
    Switch_B = 193,
    Switch_X = 194,
    Switch_Y = 195,
    Switch_LeftBumper = 196,
    Switch_RightBumper = 197,
    Switch_Plus = 198,
    Switch_Minus = 199,
    Switch_Capture = 200,
    Switch_LeftTrigger_Pull = 201,
    Switch_LeftTrigger_Click = 202,
    Switch_RightTrigger_Pull = 203,
    Switch_RightTrigger_Click = 204,
    Switch_LeftStick_Move = 205,
    Switch_LeftStick_Click = 206,
    Switch_LeftStick_DPadNorth = 207,
    Switch_LeftStick_DPadSouth = 208,
    Switch_LeftStick_DPadWest = 209,
    Switch_LeftStick_DPadEast = 210,
    Switch_RightStick_Move = 211,
    Switch_RightStick_Click = 212,
    Switch_RightStick_DPadNorth = 213,
    Switch_RightStick_DPadSouth = 214,
    Switch_RightStick_DPadWest = 215,
    Switch_RightStick_DPadEast = 216,
    Switch_DPad_North = 217,
    Switch_DPad_South = 218,
    Switch_DPad_West = 219,
    Switch_DPad_East = 220,
    SwitchProGyro_Move = 221,
    SwitchProGyro_Pitch = 222,
    SwitchProGyro_Yaw = 223,
    SwitchProGyro_Roll = 224,
    Switch_Reserved0 = 225,
    Switch_Reserved1 = 226,
    Switch_Reserved2 = 227,
    Switch_Reserved3 = 228,
    Switch_Reserved4 = 229,
    Switch_Reserved5 = 230,
    Switch_Reserved6 = 231,
    Switch_Reserved7 = 232,
    Switch_Reserved8 = 233,
    Switch_Reserved9 = 234,
    Switch_Reserved10 = 235,
    Count = 258,
    MaximumPossibleValue = 32767,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistControllerSource {
    None = 0,
    LeftTrackpad = 1,
    RightTrackpad = 2,
    Joystick = 3,
    ABXY = 4,
    Switch = 5,
    LeftTrigger = 6,
    RightTrigger = 7,
    Gyro = 8,
    CenterTrackpad = 9,
    RightJoystick = 10,
    DPad = 11,
    Key = 12,
    Mouse = 13,
    Count = 14,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistControllerSourceMode {
    None = 0,
    Dpad = 1,
    Buttons = 2,
    FourButtons = 3,
    AbsoluteMouse = 4,
    RelativeMouse = 5,
    JoystickMove = 6,
    JoystickMouse = 7,
    JoystickCamera = 8,
    ScrollWheel = 9,
    Trigger = 10,
    TouchMenu = 11,
    MouseJoystick = 12,
    MouseRegion = 13,
    RadialMenu = 14,
    SingleButton = 15,
    Switches = 16,
}

impl Default for MistControllerSourceMode {
    fn default() -> MistControllerSourceMode {
        MistControllerSourceMode::None
    }
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistSteamControllerLEDFlag {
    SetColor = 0,
    RestoreUserDefault = 1,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistSteamInputGlyphSize {
    Small = 0,
    Medium,
    Large,
    Count,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistSteamInputGlyphStyle {
    // Styles, one of
    Knockout = 0x0,
    Light = 0x1,
    Dark = 0x2,

    // Modifiers
    NeutralColorABXY = 0x10,
    SolidABXY = 0x20,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistSteamInputType {
    Unknown = 0,
    SteamController = 1,
    XBox360Controller = 2,
    XBoxOneController = 3,
    GenericXInput = 4,
    PS4Controller = 5,
    AppleMFiController = 6,
    AndroidController = 7,
    SwitchJoyConPair = 8,
    SwitchJoyConSingle = 9,
    SwitchProController = 10,
    MobileTouch = 11,
    PS3Controller = 12,
    PS5Controller = 13,
    SteamDeckController = 14,
    Count = 15,
    MaximumPossibleValue = 255,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistSteamControllerPad {
    SteamControllerPad_Left = 0,
    SteamControllerPad_Right = 1,
}

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[repr(C)]
pub enum MistControllerHapticLocation {
    Left = 1,
    Right = 2,
    Both = 1 | 2,
}

// Steam Utils

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[repr(C)]
pub enum MistGamepadTextInputLineMode {
    SingleLine = 0,
    MultipleLines = 1,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[repr(C)]
pub enum MistGamepadTextInputMode {
    Normal = 0,
    Password = 1,
}

#[derive(Serialize, Deserialize, Eq, PartialEq)]
#[repr(C)]
pub enum MistFloatingGamepadTextInputMode {
    SingleLine = 0,
    MultipleLines = 1,
    Email = 2,
    Numeric = 3,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct MistInputAnalogActionData {
    pub mode: MistControllerSourceMode,
    pub x: c_float,
    pub y: c_float,
    pub active: bool,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[repr(C)]
pub struct MistInputDigitalActionData {
    pub state: bool,
    pub active: bool,
}

#[derive(Default, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct MistInputMotionData {
    pub rot_quat_x: c_float,
    pub rot_quat_y: c_float,
    pub rot_quat_z: c_float,
    pub rot_quat_w: c_float,
    pub pos_accel_x: c_float,
    pub pos_accel_y: c_float,
    pub pos_accel_z: c_float,
    pub rot_vel_x: c_float,
    pub rot_vel_y: c_float,
    pub rot_vel_z: c_float,
}

#[derive(Clone, Copy)]
pub struct MistInputStateGamepad {
    pub input_type: MistSteamInputType,
    pub input_handle: MistInputHandle,
    pub analog_action_data: [MistInputAnalogActionData; MIST_STEAM_INPUT_MAX_ANALOG_ACTIONS + 1],
    pub digital_action_data: [MistInputDigitalActionData; MIST_STEAM_INPUT_MAX_DIGITAL_ACTIONS + 1],
    pub motion_data: MistInputMotionData,
}

impl Default for MistInputStateGamepad {
    fn default() -> MistInputStateGamepad {
        MistInputStateGamepad {
            input_type: MistSteamInputType::Unknown,
            input_handle: 0,
            analog_action_data: Default::default(),
            digital_action_data: [MistInputDigitalActionData::default();
                MIST_STEAM_INPUT_MAX_DIGITAL_ACTIONS + 1],
            motion_data: MistInputMotionData::default(),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct MistInputState {
    pub input_handles: [MistInputHandle; MIST_STEAM_INPUT_MAX_COUNT],
    pub input_handle_count: i32,
    pub gamepad_mapping: [MistInputHandle; MIST_STEAM_INPUT_MAX_COUNT],
    pub gamepads: [MistInputStateGamepad; MIST_MAX_GAMEPADS],
}

impl MistInputState {
    pub fn shmem_size() -> usize {
        std::mem::size_of::<MistInputState>() + 128 // Add some extra room for the mutex in front
    }
}
