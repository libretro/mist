typedef struct MistCallbackDlcInstalled {
  AppId app_id;
} MistCallbackDlcInstalled;

typedef struct MistCallbackSteamInputGamepadSlotChange {
  AppId app_id;
  MistInputHandle device_handle;
  uint32_t device_type;
  int32_t old_gamepad_slot;
  int32_t new_gamepad_slot;
} MistCallbackSteamInputGamepadSlotChange;

typedef struct MistCallbackRemoteStorageLocalFileChange {

} MistCallbackRemoteStorageLocalFileChange;

typedef struct MistCallbackGamepadTextInputDismissed {
  bool submitted;
  uint32_t submitted_len;
  AppId app_id;
} MistCallbackGamepadTextInputDismissed;

typedef struct MistCallbackFloatingGamepadTextInputDismissed {

} MistCallbackFloatingGamepadTextInputDismissed;

typedef struct MistCallbackAppResumingFromSuspend {

} MistCallbackAppResumingFromSuspend;

typedef struct MistCallbackSteamShutdown {

} MistCallbackSteamShutdown;

enum {
  MistCallback_DlcInstalled = 1005,
  MistCallback_SteamInputGamepadSlotChange = 2804,
  MistCallback_RemoteStorageLocalFileChange = 1333,
  MistCallback_GamepadTextInputDismissed = 714,
  MistCallback_FloatingGamepadTextInputDismissed = 738,
  MistCallback_AppResumingFromSuspend = 736,
  MistCallback_SteamShutdown = 704,
};
