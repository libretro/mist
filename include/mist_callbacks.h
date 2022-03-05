typedef struct MistCallbackDlcInstalled {
  AppId app_id;
} MistCallbackDlcInstalled;

typedef struct MistCallbackRemoteStorageLocalFileChange {

} MistCallbackRemoteStorageLocalFileChange;

typedef struct MistCallbackGamepadTextInputDismissed {
  bool submitted;
  uint32_t submitted_len;
} MistCallbackGamepadTextInputDismissed;

typedef struct MistCallbackFloatingGamepadTextInputDismissed {

} MistCallbackFloatingGamepadTextInputDismissed;

typedef struct MistCallbackAppResumingFromSuspend {

} MistCallbackAppResumingFromSuspend;

typedef struct MistCallbackSteamShutdown {

} MistCallbackSteamShutdown;

enum {
  MistCallback_DlcInstalled = 1005,
  MistCallback_RemoteStorageLocalFileChange = 1333,
  MistCallback_GamepadTextInputDismissed = 714,
  MistCallback_FloatingGamepadTextInputDismissed = 738,
  MistCallback_AppResumingFromSuspend = 736,
  MistCallback_SteamShutdown = 704,
};
