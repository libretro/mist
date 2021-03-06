enum {
	MistResult_Success = 0,
	MistResult_Mist = 1,
	MistResult_SteamApps = 100,
	MistResult_SteamFriends = 105,
	MistResult_SteamInput = 111,
	MistResult_SteamRemoteStorage = 123,
	MistResult_SteamUtils = 128
};

enum {
	MistError_InternalError = 0,
	MistError_Timeout,
	MistError_SubprocessLost = 10,
	MistError_SubprocessNotInitialized,
	MistError_SubprocessAlreadyInitialized,
	MistError_SubprocessSpawnError,
	MistError_SubprocessInitializationError,
	MistError_SubprocessUnkillable,
	MistError_SubprocessNotFound,
	MistError_InvalidString = 20
};

enum {
	SteamAppsError_InvalidDlcIndex = 0
};

enum {
	SteamFriendsError_InvalidRichPresence = 0
};

enum {
	SteamInputError_NotInitialized = 0,
	SteamInputError_ShmemError
};

enum {
	SteamRemoteStorageError_FileWriteBatchAlreadyInProgress = 0,
	SteamRemoteStorageError_FileWriteBatchNotInProgress
};

enum {
	SteamUtilsError_NoGamepadTextEntered = 0
};
