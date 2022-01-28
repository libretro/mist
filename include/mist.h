#ifndef mist_h
#define mist_h

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "mist_results.h"

#define MIST_IS_SUCCESS(result) ((result & 0xFFFF)==0)
#define MIST_IS_ERROR(result) ((result & 0xFFFF)!=0)
#define MIST_RESULT(code, error) ((MistResult)error << 16 | (MistResult)code)
#define MIST_ERROR(result) (result >> 16)
#define MIST_RESULT_CODE(result) (result & 0xFFFF)

typedef uint32_t MistResult;

typedef uint32_t AppId;

typedef struct MistDlcData {
  AppId app_id;
  bool avaliable;
  const char *name;
} MistDlcData;

typedef int32_t BuildId;

typedef uint64_t SteamId;

typedef uint32_t DepotId;

/**
 * Init mist, this is throwns an error if it was already initialised
 * Returns MistResult
 */
MistResult mist_subprocess_init(void);

/**
 * Returns the latest error
 */
const char *mist_geterror(void);

/**
 * Polls the subprocess
 * Returns MistResult
 */
MistResult mist_poll(void);

/**
 * Deinits the mist subprocess, returns false on error
 */
MistResult mist_subprocess_deinit(void);

/**
 * Get the metadata for the dlc by dlc index
 * Returns MistResult
 * dlc_data is only guaranteed to be valid til the next time the function is called
 */
MistResult mist_apps_get_dlc_data_by_index(int32_t dlc, struct MistDlcData *dlc_data);

/**
 * Checks if an app with the appid is installed
 * Returns MistResult
 */
MistResult mist_apps_is_app_installed(AppId app_id, bool *installed);

/**
 * Checks if the app is running in a cybercafe
 * Returns MistResult
 */
MistResult mist_apps_is_cybercafe(bool *is_cybercafe);

/**
 * Checks if a dlc with the appid is installed
 * Returns MistResult
 */
MistResult mist_apps_is_dlc_installed(AppId app_id, bool *installed);

/**
 * Checks if low violence mode is set
 * Returns MistResult
 */
MistResult mist_apps_is_low_violence(bool *is_low_violence);

/**
 * Checks if the active user is subscribed to the current app
 * Returns MistResult
 */
MistResult mist_apps_is_subscribed(bool *is_subscribed);

/**
 * Checks if the active user is subscribed to the app id
 * Returns MistResult
 */
MistResult mist_apps_is_subscribed_app(AppId app_id, bool *is_subscribed);

/**
 * Checks if the active user is subscribed from family sharing
 * Returns MistResult
 */
MistResult mist_apps_is_subscribed_from_family_sharing(bool *is_subscribed_from_family_sharing);

/**
 * Checks if the active user is subscribed from free weekend
 * Returns MistResult
 */
MistResult mist_apps_is_subscribed_from_free_weekend(bool *is_subscribed_from_free_weekend);

/**
 * Checks if the user has a VAC ban
 * Returns MistResult
 */
MistResult mist_apps_is_vac_banned(bool *is_vac_banned);

/**
 * Get the current build id of the application
 * Returns MistResult
 */
MistResult mist_apps_get_app_build_id(BuildId *build_id);

/**
 * Get the install dir of the app to the app id provided
 * Returns MistResult
 * app_install_dir is only guaranteed to be valid til the next time the function is called
 */
MistResult mist_apps_get_app_install_dir(AppId app_id, const char **app_install_dir);

/**
 * Get the steam id of the owner of the application
 * Returns MistResult
 */
MistResult mist_apps_get_app_owner(SteamId *steam_id);

/**
 * Get a comma seperated list of the avaliable game languages
 * Returns MistResult
 */
MistResult mist_apps_get_available_game_languages(const char **avaliable_languages);

/**
 * Get the name of the current beta, sets it to NULL if on the default beta/branch
 * current_beta_name is only guaranteed to be valid til the next time the function is called
 * Returns MistResult
 */
MistResult mist_apps_get_current_beta_name(const char **current_beta_name);

/**
 * Get the current game language
 * Returns MistResult
 */
MistResult mist_apps_get_current_game_language(const char **current_game_language);

/**
 * Get the dlc count used for getting the dlc info by index
 * Returns MistResult
 */
MistResult mist_apps_get_dlc_count(int32_t *dlc_count);

/**
 * Get the download progress of a dlc
 * Returns MistResult
 */
MistResult mist_apps_get_dlc_download_progress(AppId app_id,
                                               bool *downloading,
                                               uint64_t *bytes_downloaded,
                                               uint64_t *bytes_total);

/**
 * Get earliest purchase time for the application in unix time
 * Returns MistResult
 */
MistResult mist_apps_get_earliest_purchase_unix_time(AppId app_id, uint32_t *purchase_time);

/**
 * Writes the installed depots into a pre-allocated array named depots, sets installed_depots to the amount of depots written
 * Returns MistResult
 */
MistResult mist_apps_get_installed_depots(AppId app_id,
                                          DepotId *depots,
                                          uint32_t depots_size,
                                          uint32_t *installed_depots);

MistResult mist_apps_get_launch_command_line(const char **launch_command_line);

/**
 * Get the value of the launch query param, sets it to NULL if it does not exist
 * The value is only guaranteed to be valid til the next time the function is called
 * Returns MistResult
 */
MistResult mist_apps_get_launch_query_param(const char *key, const char **value);

/**
 * Request the dlc for the app id to be installed
 * Returns MistResult
 */
MistResult mist_apps_install_dlc(AppId app_id);

/**
 * Request a force verify of the game
 * Set missing files only to signal that a update might have been pushed
 * Returns MistResult
 */
MistResult mist_apps_mark_content_corrupt(bool missing_files_only);

/**
 * Request the dlc for the app id to be uninstalled
 * Returns MistResult
 */
MistResult mist_apps_uninstall_dlc(AppId app_id);

/**
 * Clears the rich presence key/value store
 * Returns MistResult
 */
MistResult mist_friends_clear_rich_presence(void);

/**
 * Sets the rich presence key/value
 * Value can be NULL to clear the key
 * Returns MistResult
 */
MistResult mist_friends_set_rich_presence(const char *key, const char *value);

/**
 * Begins a file write batch, use file write batches when saving files that gets stored in Steam Cloud.
 * Will error if there is already a file write batch operation in progress.
 * Returns MistResult
 */
MistResult mist_remote_storage_begin_file_write_batch(void);

/**
 * Ends a file write batch
 * Will error if there is no file write batch operation in progress.
 * Returns MistResult
 */
MistResult mist_remote_storage_end_file_write_batch(void);

/**
 * Returns the appid of the running application
 * Returns MistResult
 */
MistResult mist_utils_get_appid(uint32_t *app_id);

#endif /* mist_h */
