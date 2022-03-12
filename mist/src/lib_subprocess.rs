use parking_lot::Mutex;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use crate::{
    result::{Error, MistError},
    service::{MistClient, MistServiceToLibrary},
};

lazy_static::lazy_static! {
    pub static ref SUBPROCESS: Mutex<Option<MistSubprocess>> = Mutex::new(None);
}

macro_rules! get_subprocess {
    () => {{
        let mut lock = $crate::lib_subprocess::SUBPROCESS.lock();

        if let Some(inner) = lock.as_mut() {
            if inner.is_alive() {
                parking_lot::MutexGuard::map(lock, |inner| inner.as_mut().unwrap())
            } else {
                crate::mist_log_error("The subprocess has died");
                return crate::result::Error::Mist(crate::result::MistError::SubprocessLost).into();
            }
        } else {
            crate::mist_log_error("Subprocess has not been initialized");
            return crate::result::Error::Mist(crate::result::MistError::SubprocessNotInitialized)
                .into();
        }
    }

    /*match $crate::lib_subprocess::SUBPROCESS.lock() {
        Some(s) => {
            if s.is_alive() {
                s
            } else {
                crate::mist_log_error("The subprocess has died");
                return crate::result::Error::Mist(crate::result::MistError::SubprocessLost)
                    .into();
            }
        }
        None => {
            crate::mist_log_error("Subprocess has not been initialized");
            return crate::result::Error::Mist(
                crate::result::MistError::SubprocessNotInitialized,
            )
            .into();
        }
    }*/};
}

pub struct MistSubprocess {
    client: MistClient<ChildStdout, ChildStdin>,
    proc: Child,
}

impl MistSubprocess {
    pub fn client(&mut self) -> &mut MistClient<ChildStdout, ChildStdin> {
        &mut self.client
    }

    pub fn is_alive(&mut self) -> bool {
        self.proc
            .try_wait()
            .map(|exit| exit.is_none())
            .unwrap_or(false)
    }
}

pub fn mist_init_subprocess() -> Result<(), Error> {
    {
        if SUBPROCESS.lock().is_some() {
            crate::mist_log_error("The subprocess has already been initialized");
            return Err(Error::Mist(MistError::SubprocessAlreadyInitialized));
        }
    }

    let exe = if cfg!(unix) {
        "mist"
    } else if cfg!(windows) {
        "mist.exe"
    } else {
        panic!("[mist] unsupported platform")
    };

    let (exe_cwd, exe_path) = match std::env::current_dir() {
        Ok(p) => {
            let exe_cwd = p.join("mist");
            let exe_path = exe_cwd.join(exe);
            (exe_cwd, exe_path)
        }
        Err(err) => {
            crate::mist_log_error(&format!("Invalid current path: {}", err));
            return Err(Error::Mist(MistError::SubprocessNotFound));
        }
    };

    let exe_cwd_str = exe_cwd.to_string_lossy().to_string();
    let ld_library_path = std::env::var("LD_LIBRARY_PATH")
        .map(|p| p + ":" + &exe_cwd_str)
        .unwrap_or_else(|_| exe_cwd_str);

    let mut proc = match Command::new(exe_path)
        .current_dir(exe_cwd)
        .arg(crate::consts::PROCESS_INIT_SECRET)
        .env("LD_LIBRARY_PATH", ld_library_path.as_str())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            crate::mist_log_error(&format!("Error spawning subprocess: {}", err));
            return Err(Error::Mist(MistError::SubprocessSpawnError));
        }
    };

    let client = MistClient::create(proc.stdout.take().unwrap(), proc.stdin.take().unwrap());
    let subprocess = MistSubprocess { client, proc };

    // Set the subprocess
    *SUBPROCESS.lock() = Some(subprocess);

    // Now let's wait for the init
    let subprocess = get_subprocess!();

    // Wait for the subprocess to initialize
    match subprocess
        .client
        .receiver
        .recv_timeout(std::time::Duration::from_secs(4))
    {
        Ok(msg) => match msg {
            MistServiceToLibrary::Initialized => (),
            MistServiceToLibrary::InitError(err) => {
                crate::mist_log_error(&format!("Subprocess initialization error: {}", err));
                return Err(Error::Mist(MistError::SubprocessInitializationError));
            }
            _ => unreachable!(),
        },
        Err(err) => {
            crate::mist_log_error(&format!("Subprocess initialization error: {}", err));
            return Err(Error::Mist(MistError::SubprocessInitializationError));
        }
    }

    Ok(())
}

pub fn mist_deinit_subprocess() -> Result<(), Error> {
    let mut subprocess = match SUBPROCESS.lock().take() {
        Some(s) => s,
        None => {
            crate::mist_log_error(
                "The subprocess cannot be deinitialized when it has not been initialized.",
            );
            return Err(Error::Mist(MistError::SubprocessNotInitialized));
        }
    };

    // Tell the subprocess to terminate
    subprocess.client().internal().exit()?;

    // Give it 500ms to terminate before killing the process
    let mut exited = false;
    for _ in 0..10 {
        if subprocess
            .proc
            .try_wait()
            .map(|e| e.is_some())
            .unwrap_or(false)
        {
            exited = true;
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }

    if !exited {
        match subprocess.proc.kill() {
            Ok(_) => (),
            Err(err) => {
                crate::mist_log_error(&format!("Error killing the subprocess: {}", err));
                return Err(Error::Mist(MistError::SubprocessUnkillable));
            }
        }
    }

    Ok(())
}
