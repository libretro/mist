#![windows_subsystem = "windows"]

#[macro_use]
mod codegen;
mod callbacks;
mod consts;
mod result;
mod service;
mod types;

use consts::PROCESS_INIT_SECRET;

mod subprocess;

fn main() {
    // Keep users away
    if std::env::args()
        .nth(1)
        .map(|e| e != PROCESS_INIT_SECRET)
        .unwrap_or(true)
    {
        eprintln!("This executable is a subprocess used for steam integration and is not designed to be launched manually.");
        std::process::exit(1);
    }

    // Init the steam api
    unsafe {
        if !steamworks_sys::SteamAPI_Init() {
            eprintln!("[mist] Error during SteamAPI init.");
            std::process::exit(1);
        }

        // Setup manual dispatch since we are not using c++ classes
        steamworks_sys::SteamAPI_ManualDispatch_Init();
    }

    if let Err(err) = subprocess::run() {
        eprintln!("[mist] Error while running subprocess: {}", err);
        std::process::exit(1);
    }

    std::process::exit(0);
}
