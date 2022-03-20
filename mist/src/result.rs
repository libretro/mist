use serde_derive::{Deserialize, Serialize};

pub type MistResult = u32;

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
/// cbindgen:ignore
pub const Success: MistResult = 0;

mist_errors! {
    // Mist errors
    Mist: 1 {
        InternalError = 0,
        Timeout,
        SubprocessLost = 10,
        SubprocessNotInitialized,
        SubprocessAlreadyInitialized,
        SubprocessSpawnError,
        SubprocessInitializationError,
        SubprocessUnkillable,
        SubprocessNotFound,
        InvalidString = 20
    },
    SteamApps: 100 {
        InvalidDlcIndex = 0
    },
    SteamFriends: 105 {
        InvalidRichPresence = 0
    },
    SteamInput: 111 {
        NotInitialized = 0,
        ShmemError
    },
    SteamRemoteStorage: 123 {
        FileWriteBatchAlreadyInProgress = 0,
        FileWriteBatchNotInProgress
    },
    SteamUtils: 128 {
        NoGamepadTextEntered = 0
    }
}
