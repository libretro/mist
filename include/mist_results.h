enum {
	MistResult_Success = 0,
	MistResult_Mist = 1,
	MistResult_SteamApps = 100
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
