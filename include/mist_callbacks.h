typedef struct MistCallbackDlcInstalled {
  AppId app_id;
} MistCallbackDlcInstalled;

typedef struct MistCallbackRemoteStorageLocalFileChange {

} MistCallbackRemoteStorageLocalFileChange;

enum {
  MistCallback_DlcInstalled = 1005,
  MistCallback_RemoteStorageLocalFileChange = 1333,
};
