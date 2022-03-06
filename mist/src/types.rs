use serde_derive::{Deserialize, Serialize};

pub type AppId = u32;
pub type BuildId = i32;
pub type DepotId = u32;
pub type SteamId = u64;
pub type SteamUser = i32;

#[derive(Serialize, Deserialize, Eq, PartialEq)]
pub struct DlcData {
    pub app_id: AppId,
    pub avaliable: bool,
    pub name: String,
}

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
