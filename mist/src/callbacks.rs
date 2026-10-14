use crate::types::*;

mist_callbacks!(
    SteamApps {
        DlcInstalled {
            m_nAppID => app_id: AppId
        }
    },
    SteamInput {
        SteamInputGamepadSlotChange {
            m_unAppID => app_id: AppId,
            m_ulDeviceHandle => device_handle: MistInputHandle,
            m_eDeviceType => device_type: u32,
            m_nOldGamepadSlot => old_gamepad_slot: i32,
            m_nNewGamepadSlot => new_gamepad_slot: i32
        }
    },
    SteamRemoteStorage {
        RemoteStorageLocalFileChange {}
    },
    SteamUtils {
        GamepadTextInputDismissed {
            m_bSubmitted => submitted: bool,
            m_unSubmittedText => submitted_len: u32,
            m_unAppID => app_id: AppId,
            [(|server, event| {
                if !event.m_bSubmitted { return; }

                let steam_utils = unsafe { steamworks_sys::SteamAPI_SteamUtils_v011() };

                let text_len = unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetEnteredGamepadTextLength(steam_utils) };

                let mut input = String::with_capacity(text_len as usize);

                unsafe { steamworks_sys::SteamAPI_ISteamUtils_GetEnteredGamepadTextInput(steam_utils, input.as_mut_ptr() as *mut i8, text_len) };

                server.service().entered_gamepad_text = Some(input);
            })]
        },
        FloatingGamepadTextInputDismissed {},
        AppResumingFromSuspend {},
        SteamShutdown {}
    }
);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct MistCallbackMsg {
    pub user: SteamUser,
    pub callback: u32,
    pub data: *const std::ffi::c_void,
}
