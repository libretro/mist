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
