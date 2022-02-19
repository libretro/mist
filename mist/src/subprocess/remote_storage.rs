use super::MistServerService;
use crate::{
    result::{Error, SteamRemoteStorageError},
    service::MistServiceSteamRemoteStorage,
};

// ISteamRemoteStorage
impl MistServiceSteamRemoteStorage for MistServerService {
    fn begin_file_write_batch(&mut self) -> Result<(), Error> {
        if unsafe {
            steamworks_sys::SteamAPI_ISteamRemoteStorage_BeginFileWriteBatch(
                self.steam_remote_storage,
            )
        } {
            Ok(())
        } else {
            Err(Error::SteamRemoteStorage(
                SteamRemoteStorageError::FileWriteBatchAlreadyInProgress,
            ))
        }
    }
    fn end_file_write_batch(&mut self) -> Result<(), Error> {
        if unsafe {
            steamworks_sys::SteamAPI_ISteamRemoteStorage_EndFileWriteBatch(
                self.steam_remote_storage,
            )
        } {
            Ok(())
        } else {
            Err(Error::SteamRemoteStorage(
                SteamRemoteStorageError::FileWriteBatchNotInProgress,
            ))
        }
    }
}
