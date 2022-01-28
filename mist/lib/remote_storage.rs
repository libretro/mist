use crate::result::{MistResult, Success};

/// Begins a file write batch, use file write batches when saving files that gets stored in Steam Cloud.
/// Will error if there is already a file write batch operation in progress.
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_remote_storage_begin_file_write_batch() -> MistResult {
    let subprocess = get_subprocess!();

    unwrap_client_result!(subprocess
        .client()
        .remote_storage()
        .begin_file_write_batch());

    Success
}

/// Ends a file write batch
/// Will error if there is no file write batch operation in progress.
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_remote_storage_end_file_write_batch() -> MistResult {
    let subprocess = get_subprocess!();

    unwrap_client_result!(subprocess.client().remote_storage().end_file_write_batch());

    Success
}
