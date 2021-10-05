use std::str::FromStr;

fn main() {
    let apps = unsafe {
        if !steamworks_sys::SteamAPI_Init() {
            panic!("[mist] Error initializing SteamAPI");
        }

        // Setup manual dispatch since we are not using c++ classes
        steamworks_sys::SteamAPI_ManualDispatch_Init();

        steamworks_sys::SteamAPI_SteamApps_v008()
    };

    let app_id = match std::env::args().nth(1) {
        Some(arg) => u32::from_str(&arg).expect("Valid appid"),
        None => panic!("[mist] Missing appid argument"),
    };

    let mut folder = vec![0; 2048];
    let len = unsafe {
        steamworks_sys::SteamAPI_ISteamApps_GetAppInstallDir(
            apps,
            app_id,
            folder.as_mut_ptr(),
            folder.len() as u32,
        )
    };
    // Get rid of steamworks
    unsafe { steamworks_sys::SteamAPI_Shutdown() };

    if len == 0 {
        std::process::exit(1);
    } else {
        println!(
            "{}",
            unsafe { std::ffi::CStr::from_ptr(folder.as_ptr()) }.to_string_lossy()
        )
    }
}
