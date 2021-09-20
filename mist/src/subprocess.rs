use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use crate::service::{MistClient, MistServiceToLibrary};

static mut SUBPROCESS: Option<MistSubprocess> = None;

macro_rules! get_subprocess {
    () => {
        match crate::subprocess::mist_get_subprocess() {
            Some(s) => s,
            None => {
                crate::mist_set_error("Subprocess has not been initialized");
                return true;
            }
        };
    };
}

pub struct MistSubprocess {
    client: MistClient<ChildStdout, ChildStdin>,
    proc: Child,
}

impl MistSubprocess {
    pub fn client(&mut self) -> &mut MistClient<ChildStdout, ChildStdin> {
        &mut self.client
    }
}

pub fn mist_init_subprocess() -> bool {
    if unsafe { SUBPROCESS.is_some() } {
        crate::mist_set_error("The subprocess has already been initialized");
        return true;
    }

    let exe = if cfg!(unix) {
        "mist"
    } else if cfg!(windows) {
        "mist.exe"
    } else {
        panic!("unsupported platform")
    };

    let (exe_cwd, exe_path) = match std::env::current_dir() {
        Ok(p) => {
            let exe_cwd = p.join("mist");
            let exe_path = exe_cwd.join(exe);
            (exe_cwd, exe_path)
        }
        Err(err) => {
            crate::mist_set_error(&format!("Invalid current path: {}", err));
            return true;
        }
    };

    let mut proc = match Command::new(exe_path)
        .current_dir(exe_cwd)
        .arg(crate::consts::PROCESS_INIT_SECRET)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => {
            crate::mist_set_error(&format!("Error spawning subprocess: {}", err));
            return true;
        }
    };

    let client = MistClient::create(proc.stdout.take().unwrap(), proc.stdin.take().unwrap());
    let subprocess = MistSubprocess { client, proc };

    // Set the subprocess
    unsafe { SUBPROCESS = Some(subprocess) };

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
                crate::mist_set_error(&format!("Subprocess initialization error: {}", err));
                return true;
            }
            _ => unreachable!(),
        },
        Err(err) => {
            crate::mist_set_error(&format!("Subprocess initialization error: {}", err));
            return true;
        }
    }

    false
}

pub fn mist_deinit_subprocess() -> bool {
    let subprocess = match unsafe { &mut SUBPROCESS } {
        Some(s) => s,
        None => {
            crate::mist_set_error(
                "The subprocess cannot be deinitialized when it has not been initialized.",
            );
            return true;
        }
    };

    // Tell the subprocess to terminate
    // TODO: Kill the subprocess to terminate
    //subprocess.proc.terminate().ok();

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
                crate::mist_set_error(&format!("Error killing the subprocess: {}", err));
                return true;
            }
        }
    }

    unsafe { SUBPROCESS = None };

    false
}

pub fn mist_get_subprocess<'a>() -> Option<&'a mut MistSubprocess> {
    unsafe { SUBPROCESS.as_mut() }
}