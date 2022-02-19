use crate::{
    result::{MistResult, Success},
    types::AppId,
};

/// Returns the appid of the running application
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_utils_get_appid(app_id: *mut AppId) -> MistResult {
    let subprocess = get_subprocess!();

    let id = unwrap_client_result!(subprocess.client().steam_utils().get_appid());

    unsafe {
        *app_id = id;
    }

    Success
}
