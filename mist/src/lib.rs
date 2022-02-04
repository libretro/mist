#[macro_use]
mod codegen;
mod consts;
pub mod result;
mod service;
#[macro_use]
mod lib_subprocess;
mod types;

use crate::result::{Error, MistError, MistResult, Success};

macro_rules! unwrap_client_result {
    ($res:expr) => {
        match $res {
            Ok(res) => res,
            Err(err) => {
                return err.into();
            }
        }
    };
}

pub fn mist_log_error(err: &str) {
    eprintln!("[mist] {}", err);
}

/// Init mist, this is throwns an error if it was already initialised
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_subprocess_init() -> MistResult {
    let result = std::panic::catch_unwind(lib_subprocess::mist_init_subprocess);

    match result {
        Ok(res) => unwrap_client_result!(res),
        Err(_) => {
            mist_log_error("Internal panic during initialization");
            return Error::Mist(MistError::SubprocessNotInitialized).into();
        }
    }

    Success
}

/// Polls the subprocess
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_poll() -> MistResult {
    let _subprocess = get_subprocess!();
    Success
}

#[path = "../lib/apps.rs"]
mod apps;
#[path = "../lib/friends.rs"]
mod friends;
#[path = "../lib/remote_storage.rs"]
mod remote_storage;
#[path = "../lib/utils.rs"]
mod utils;

/// Deinits the mist subprocess, returns false on error
#[no_mangle]
pub extern "C" fn mist_subprocess_deinit() -> MistResult {
    unwrap_client_result!(lib_subprocess::mist_deinit_subprocess());

    Success
}
