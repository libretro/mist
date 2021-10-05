use std::ffi::{CStr, CString};

use super::MistServerService;
use crate::{service::MistServiceApps, types::*};

// ISteamApps
impl MistServiceApps for MistServerService {
    fn get_dlc_data_by_index(&mut self, dlc: i32) -> Option<DlcData> {
        let mut app_id = 0;
        let mut avaliable = false;
        let mut name = vec![0; 2048];

        let ok = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_BGetDLCDataByIndex(
                self.steam_apps,
                dlc,
                &mut app_id,
                &mut avaliable,
                name.as_mut_ptr(),
                name.len() as i32,
            )
        };

        if ok {
            Some(DlcData {
                app_id,
                avaliable,
                name: unsafe { CStr::from_ptr(name.as_ptr()) }
                    .to_string_lossy()
                    .into(),
            })
        } else {
            None
        }
    }

    fn is_app_installed(&mut self, app_id: AppId) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsAppInstalled(self.steam_apps, app_id) }
    }

    fn is_cybercafe(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsCybercafe(self.steam_apps) }
    }

    fn is_dlc_installed(&mut self, app_id: AppId) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsDlcInstalled(self.steam_apps, app_id) }
    }

    fn is_low_violence(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsLowViolence(self.steam_apps) }
    }

    fn is_subscribed(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsSubscribed(self.steam_apps) }
    }

    fn is_subscribed_app(&mut self, app_id: AppId) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsSubscribedApp(self.steam_apps, app_id) }
    }

    fn is_subscribed_from_family_sharing(&mut self) -> bool {
        unsafe {
            steamworks_sys::SteamAPI_ISteamApps_BIsSubscribedFromFamilySharing(self.steam_apps)
        }
    }

    fn is_subscribed_from_free_weekend(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsSubscribedFromFreeWeekend(self.steam_apps) }
    }

    fn is_vac_banned(&mut self) -> bool {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_BIsVACBanned(self.steam_apps) }
    }

    fn get_app_build_id(&mut self) -> BuildId {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_GetAppBuildId(self.steam_apps) }
    }

    fn get_app_install_dir(&mut self, app_id: AppId) -> Option<String> {
        let mut folder = vec![0; 2048];
        let len = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetAppInstallDir(
                self.steam_apps,
                app_id,
                folder.as_mut_ptr(),
                folder.len() as u32,
            )
        };

        if len == 0 {
            None
        } else {
            Some(
                unsafe { CStr::from_ptr(folder.as_ptr()) }
                    .to_string_lossy()
                    .into(),
            )
        }
    }

    fn get_app_owner(&mut self) -> SteamId {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_GetAppOwner(self.steam_apps) }
    }

    fn get_available_game_languages(&mut self) -> String {
        let languages = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetAvailableGameLanguages(self.steam_apps)
        };

        unsafe { CStr::from_ptr(languages) }
            .to_string_lossy()
            .into()
    }

    fn get_current_beta_name(&mut self) -> Option<String> {
        let mut beta = vec![0; 2048];
        let on_beta = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetCurrentBetaName(
                self.steam_apps,
                beta.as_mut_ptr(),
                beta.len() as i32,
            )
        };

        if on_beta {
            Some(
                unsafe { CStr::from_ptr(beta.as_ptr()) }
                    .to_string_lossy()
                    .into(),
            )
        } else {
            None
        }
    }

    fn get_current_game_language(&mut self) -> String {
        let language =
            unsafe { steamworks_sys::SteamAPI_ISteamApps_GetCurrentGameLanguage(self.steam_apps) };

        unsafe { CStr::from_ptr(language) }.to_string_lossy().into()
    }

    fn get_dlc_count(&mut self) -> i32 {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_GetDLCCount(self.steam_apps) }
    }

    fn get_dlc_download_progress(&mut self, app_id: AppId) -> Option<(u64, u64)> {
        let mut bytes_downloaded = 0;
        let mut total_bytes = 0;

        if unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetDlcDownloadProgress(
                self.steam_apps,
                app_id,
                &mut bytes_downloaded,
                &mut total_bytes,
            )
        } {
            Some((bytes_downloaded, total_bytes))
        } else {
            None
        }
    }

    fn get_earliest_purchase_unix_time(&mut self, app_id: AppId) -> u32 {
        unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetEarliestPurchaseUnixTime(self.steam_apps, app_id)
        }
    }

    //fn get_file_details(file_name: String) -> ();

    fn get_installed_depots(&mut self, app_id: AppId) -> Vec<DepotId> {
        let mut depots = vec![0; 2048];
        let depots_len = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetInstalledDepots(
                self.steam_apps,
                app_id,
                depots.as_mut_ptr(),
                depots.len() as u32,
            )
        };

        depots[..depots_len as usize].to_vec()
    }

    fn get_launch_command_line(&mut self) -> String {
        let mut launch_command_line = vec![0; 2048];
        let len = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetLaunchCommandLine(
                self.steam_apps,
                launch_command_line.as_mut_ptr(),
                launch_command_line.len() as i32,
            )
        };

        if len == 0 {
            "".into()
        } else {
            unsafe { CStr::from_ptr(launch_command_line.as_ptr()) }
                .to_string_lossy()
                .into()
        }
    }

    fn get_launch_query_param(&mut self, key: String) -> Option<String> {
        static mut LAUNCH_QUERY_KEY: Option<CString> = None;
        unsafe {
            LAUNCH_QUERY_KEY = Some(CString::new(key).ok()?);
        }

        let param_ptr = unsafe {
            steamworks_sys::SteamAPI_ISteamApps_GetLaunchQueryParam(
                self.steam_apps,
                LAUNCH_QUERY_KEY.as_ref().unwrap().as_ptr(),
            )
        };

        let param = unsafe { CStr::from_ptr(param_ptr) }.to_string_lossy();

        if param.len() == 0 {
            None
        } else {
            Some(param.into())
        }
    }

    fn install_dlc(&mut self, app_id: AppId) {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_InstallDLC(self.steam_apps, app_id) }
    }

    fn mark_content_corrupt(&mut self, missing_files_only: bool) -> bool {
        unsafe {
            steamworks_sys::SteamAPI_ISteamApps_MarkContentCorrupt(
                self.steam_apps,
                missing_files_only,
            )
        }
    }

    fn uninstall_dlc(&mut self, app_id: AppId) {
        unsafe { steamworks_sys::SteamAPI_ISteamApps_UninstallDLC(self.steam_apps, app_id) }
    }
}
