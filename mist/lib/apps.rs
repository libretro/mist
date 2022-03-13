use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use crate::{
    result::{MistResult, Success},
    types::*,
};

/// Get the metadata for the dlc by dlc index
/// Returns MistResult
/// dlc_data is only guaranteed to be valid til the next time the function is called
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_dlc_data_by_index(
    dlc: i32,
    app_id: *mut AppId,
    availiable: *mut bool,
    name: *mut c_char,
    name_size: u32,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let dlc = unwrap_client_result!(subprocess.client().steam_apps().get_dlc_data_by_index(dlc));
    let name_cstr = CString::new(dlc.name).unwrap_or_default();

    unsafe {
        *app_id = dlc.app_id;
        *availiable = dlc.avaliable;
        crate::copy_string_out(&name_cstr, name, name_size as _);
    }

    Success
}

/// Checks if an app with the appid is installed
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_app_installed(
    app_id: AppId,
    installed: *mut bool,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *installed =
            unwrap_client_result!(subprocess.client().steam_apps().is_app_installed(app_id))
    };

    Success
}

/// Checks if the app is running in a cybercafe
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_cybercafe(is_cybercafe: *mut bool) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_cybercafe = unwrap_client_result!(subprocess.client().steam_apps().is_cybercafe())
    };

    Success
}

/// Checks if a dlc with the appid is installed
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_dlc_installed(
    app_id: AppId,
    installed: *mut bool,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *installed =
            unwrap_client_result!(subprocess.client().steam_apps().is_dlc_installed(app_id))
    };
    Success
}

/// Checks if low violence mode is set
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_low_violence(is_low_violence: *mut bool) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_low_violence = unwrap_client_result!(subprocess.client().steam_apps().is_low_violence())
    };
    Success
}

/// Checks if the active user is subscribed to the current app
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_subscribed(is_subscribed: *mut bool) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_subscribed = unwrap_client_result!(subprocess.client().steam_apps().is_subscribed())
    };
    Success
}

/// Checks if the active user is subscribed to the app id
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_subscribed_app(
    app_id: AppId,
    is_subscribed: *mut bool,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_subscribed =
            unwrap_client_result!(subprocess.client().steam_apps().is_subscribed_app(app_id))
    };
    Success
}

/// Checks if the active user is subscribed from family sharing
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_subscribed_from_family_sharing(
    is_subscribed_from_family_sharing: *mut bool,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_subscribed_from_family_sharing = unwrap_client_result!(subprocess
            .client()
            .steam_apps()
            .is_subscribed_from_family_sharing())
    };
    Success
}

/// Checks if the active user is subscribed from free weekend
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_subscribed_from_free_weekend(
    is_subscribed_from_free_weekend: *mut bool,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_subscribed_from_free_weekend = unwrap_client_result!(subprocess
            .client()
            .steam_apps()
            .is_subscribed_from_free_weekend())
    };
    Success
}

/// Checks if the user has a VAC ban
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_is_vac_banned(is_vac_banned: *mut bool) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *is_vac_banned = unwrap_client_result!(subprocess.client().steam_apps().is_vac_banned())
    };
    Success
}

/// Get the current build id of the application
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_app_build_id(build_id: *mut BuildId) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *build_id = unwrap_client_result!(subprocess.client().steam_apps().get_app_build_id())
    };
    Success
}

/// Get the install dir of the app to the app id provided
/// Returns MistResult
/// app_install_dir is only guaranteed to be valid til the next time the function is called
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_app_install_dir(
    app_id: AppId,
    folder: *mut c_char,
    folder_size: u32,
    folder_copied: *mut u32,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let install_dir =
        unwrap_client_result!(subprocess.client().steam_apps().get_app_install_dir(app_id));

    match install_dir {
        Some(install) => {
            let install_cstr = CString::new(install).unwrap_or_default();

            unsafe {
                *folder_copied =
                    crate::copy_string_out(&install_cstr, folder, folder_size as _) as u32;
            }

            Success
        }
        None => Success,
    }
}

/// Get the steam id of the owner of the application
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_app_owner(steam_id: *mut SteamId) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe { *steam_id = unwrap_client_result!(subprocess.client().steam_apps().get_app_owner()) };
    Success
}

/// Get a comma seperated list of the avaliable game languages
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_available_game_languages(
    avaliable_languages: *mut *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    if let Some(langs) = &subprocess.state().avaliable_languages {
        unsafe {
            *avaliable_languages = langs.as_ptr();
        }
    } else {
        let game_languages = unwrap_client_result!(subprocess
            .client()
            .steam_apps()
            .get_available_game_languages());

        let game_languages_cstr = CString::new(game_languages).unwrap_or_default();

        unsafe {
            *avaliable_languages = game_languages_cstr.as_ptr();
        }

        subprocess.state_mut().avaliable_languages = Some(game_languages_cstr);
    }

    Success
}

/// Get the name of the current beta, sets it to NULL if on the default beta/branch
/// current_beta_name is only guaranteed to be valid til the next time the function is called
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_current_beta_name(
    on_beta: *mut bool,
    name: *mut c_char,
    name_size: u32,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    let beta = unwrap_client_result!(subprocess.client().steam_apps().get_current_beta_name());

    match beta {
        Some(beta) => unsafe {
            *on_beta = true;

            let beta_cstr = CString::new(beta).unwrap_or_default();

            crate::copy_string_out(&beta_cstr, name, name_size as _);
        },
        None => unsafe {
            *on_beta = false;
        },
    }

    Success
}

/// Get the current game language
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_current_game_language(
    current_game_language: *mut *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();

    if let Some(lang) = &subprocess.state().current_language {
        unsafe {
            *current_game_language = lang.as_ptr();
        }
    } else {
        let current_language =
            unwrap_client_result!(subprocess.client().steam_apps().get_current_game_language());

        let current_language_cstr = CString::new(current_language).unwrap_or_default();

        unsafe {
            *current_game_language = current_language_cstr.as_ptr();
        }

        subprocess.state_mut().current_language = Some(current_language_cstr);
    }

    Success
}

/// Get the dlc count used for getting the dlc info by index
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_dlc_count(dlc_count: *mut i32) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe { *dlc_count = unwrap_client_result!(subprocess.client().steam_apps().get_dlc_count()) };
    Success
}

/// Get the download progress of a dlc
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_dlc_download_progress(
    app_id: AppId,
    downloading: *mut bool,
    bytes_downloaded: *mut u64,
    bytes_total: *mut u64,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let download_progress = unwrap_client_result!(subprocess
        .client()
        .steam_apps()
        .get_dlc_download_progress(app_id));

    if let Some((downloaded, total)) = download_progress {
        unsafe {
            *downloading = true;
            *bytes_downloaded = downloaded;
            *bytes_total = total;
        }
    } else {
        unsafe {
            *downloading = false;
        }
    }

    Success
}

/// Get earliest purchase time for the application in unix time
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_earliest_purchase_unix_time(
    app_id: AppId,
    purchase_time: *mut u32,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    unsafe {
        *purchase_time = unwrap_client_result!(subprocess
            .client()
            .steam_apps()
            .get_earliest_purchase_unix_time(app_id))
    };
    Success
}

//#[no_mangle]
//pub extern "C" fn mist_steam_apps_get_file_details(file_name: String) -> ();

/// Writes the installed depots into a pre-allocated array named depots, sets installed_depots to the amount of depots written
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_installed_depots(
    app_id: AppId,
    depots: *mut DepotId,
    depots_size: u32,
    installed_depots: *mut u32,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let depot_ids = unwrap_client_result!(subprocess
        .client()
        .steam_apps()
        .get_installed_depots(app_id));

    unsafe {
        let count = depots_size.min(depot_ids.len() as u32);
        std::ptr::copy_nonoverlapping(depot_ids.as_ptr(), depots, count as usize);
        *installed_depots = count;
    }

    Success
}

#[no_mangle]
pub extern "C" fn mist_steam_apps_get_launch_command_line(
    command_line: *mut c_char,
    command_line_size: u32,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let launch_command =
        unwrap_client_result!(subprocess.client().steam_apps().get_launch_command_line());

    let launch_command_cstr = CString::new(launch_command).unwrap_or_default();

    unsafe {
        crate::copy_string_out(&launch_command_cstr, command_line, command_line_size as _);
    }

    Success
}

/// Get the value of the launch query param, sets it to NULL if it does not exist
/// The value is only guaranteed to be valid til the next time the function is called
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_get_launch_query_param(
    key: *const c_char,
    value: *mut *const c_char,
) -> MistResult {
    let mut subprocess = get_subprocess!();
    let key = unsafe { CStr::from_ptr(key) }.to_string_lossy().to_string();

    if let Some(param_value_cstr) = subprocess.state().launch_query_params.get(&key) {
        unsafe {
            *value = param_value_cstr.as_ptr();
        }
    } else {
        let param_value = unwrap_client_result!(subprocess
            .client()
            .steam_apps()
            .get_launch_query_param(key.clone()));

        match param_value {
            Some(param_value) => {
                let param_value_cstr = CString::new(param_value).unwrap_or_default();

                unsafe {
                    *value = param_value_cstr.as_ptr();
                }

                subprocess
                    .state_mut()
                    .launch_query_params
                    .insert(key, param_value_cstr);
            }
            None => unsafe {
                const EMPTY: [c_char; 1] = [0];
                *value = &EMPTY as *const c_char;
            },
        }
    }

    Success
}

/// Request the dlc for the app id to be installed
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_install_dlc(app_id: AppId) -> MistResult {
    let mut subprocess = get_subprocess!();
    unwrap_client_result!(subprocess.client().steam_apps().install_dlc(app_id));
    Success
}

/// Request a force verify of the game
/// Set missing files only to signal that a update might have been pushed
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_mark_content_corrupt(missing_files_only: bool) -> MistResult {
    let mut subprocess = get_subprocess!();
    unwrap_client_result!(subprocess
        .client()
        .steam_apps()
        .mark_content_corrupt(missing_files_only));
    Success
}

/// Request the dlc for the app id to be uninstalled
/// Returns MistResult
#[no_mangle]
pub extern "C" fn mist_steam_apps_uninstall_dlc(app_id: AppId) -> MistResult {
    let mut subprocess = get_subprocess!();
    unwrap_client_result!(subprocess.client().steam_apps().uninstall_dlc(app_id));
    Success
}
