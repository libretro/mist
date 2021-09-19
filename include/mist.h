#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Setups mist, this is a no-op if it is already running, returns true on error
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
 * Deinits the runtime, returns true on error
 */
bool mist_deinit(void);
