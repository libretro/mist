#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Init mist, this is throwns an error if it was already initialised, returns true on error
 */
bool mist_init(void);

/**
 * Returns the latest error
 */
const char *mist_geterror(void);

/**
 * Polls the subprocess, returns true on error
 */
bool mist_poll(void);

/**
 * Clears the rich presence key/value store
 */
bool mist_clear_rich_presence(void);

/**
 * Sets the rich presence key/value
 * Value can be NULL to clear the key
 */
bool mist_set_rich_presence(const int8_t *key, const int8_t *value);

/**
 * Returns the appid of the running application
 */
bool mist_get_appid(uint32_t *app_id);

/**
 * Deinits the runtime, returns true on error
 */
bool mist_deinit(void);
