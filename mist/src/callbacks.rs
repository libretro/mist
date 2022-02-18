use crate::types::*;

mist_callbacks!(
    SteamApps {
        DlcInstalled {
            m_nAppID => app_id: AppId
        }
    },
    SteamRemoteStorage {
        RemoteStorageLocalFileChange {}
    }
);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MistCallbackMsg {
    pub user: SteamUser,
    pub callback: u32,
    pub data: *const std::ffi::c_void,
}
