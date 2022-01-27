use crate::result::{MistResult, Success};

/// Returns the appid of the running application
/// Returns
#[no_mangle]
pub extern "C" fn mist_utils_get_appid(app_id: *mut u32) -> MistResult {
    let subprocess = get_subprocess!();

    let id = unwrap_client_result!(subprocess.client().utils().get_appid());

    unsafe {
        *app_id = id;
    }

    Success
}
