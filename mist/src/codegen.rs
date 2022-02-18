macro_rules! mist_log_error {
    ($error:expr) => {
        #[cfg(not(feature = "steamworks"))]
        crate::mist_log_error($error);
        // TODO: Set the error in some way on the subprocess
        #[cfg(feature = "steamworks")]
        drop($error);
    };
}

// I know this macro might look scary, but it abstracts away all the painful IPC protocol work
macro_rules! mist_service {
    (__fallback_ty, $ty:ty) => {
        $ty
    };
    (__fallback_ty) => {
        ()
    };
    (__fallback_ty_ret, $ty:ty) => {

    };
    (__fallback_ty_ret) => {
        return Ok(());
    };
    (__fallback_ty_ret, $call_name:ident, $res:ident, $ty:ty) => {
        MistServiceToLibraryResult::$call_name($res)
    };
    (__fallback_ty_ret, $call_name:ident, $res:ident) => {
        MistServiceToLibraryResult::$call_name
    };
    ($($module:ident {
        $(fn $call_name:ident($($arg:ident : $arg_ty:ty),*)$(-> $return_ty:ty)?;)*
    })*) => {
        paste::paste! {
            use anyhow::Result;
            use crate::result::{Error, MistError};
            use serde_derive::{Serialize, Deserialize};
            // Some are just used for client, and some just for server
            #[allow(unused_imports)]
            use std::{io::{Read, Write}, time::Duration};

            $(
                // Trait for subprocess
                pub trait [<MistService $module>] {
                    $(
                        fn $call_name(&mut self $(, $arg : $arg_ty)*) -> Result<mist_service!(__fallback_ty$(,$return_ty)?), Error>;
                    )+
                }

                // Trait for client/library
                pub trait [<MistClient $module>] {
                    $(
                        fn $call_name(&mut self $(, $arg : $arg_ty)*) -> Result<mist_service!(__fallback_ty$(,$return_ty)?), Error>;
                    )+
                }
            )+

            pub trait MistService: $( [<MistService $module>] + )+ {}

            #[allow(dead_code)]
            pub struct MistClient<R: Read, W: Write> {
                callbacks: std::collections::VecDeque<crate::callbacks::MistCallback>,
                write: W,
                pub receiver: std::sync::mpsc::Receiver<MistServiceToLibrary>,
                _read: std::marker::PhantomData<R>,
            }

            #[allow(dead_code)]
            impl<R: Read + Send + 'static, W: Write> MistClient<R, W> {
                pub fn create(mut read: R, write: W) -> MistClient<R, W> {
                    let (sender, receiver) = std::sync::mpsc::channel::<MistServiceToLibrary>();
                    // Spawn a stdin listen thread
                    std::thread::spawn(move || {
                        loop {
                            let mut len_buf = [0u8; 32 / 8];
                            match read.read_exact(&mut len_buf) {
                                Ok(_) => {

                                    let len = u32::from_le_bytes(len_buf) as usize;
                                    let mut msg_buf = vec![0; len];
                                    if let Err(err) = read.read_exact(&mut msg_buf) {
                                        eprintln!("[mist] Error reading data payload from subprocess: {}", err);
                                    }

                                    match bincode::deserialize(&msg_buf) {
                                        Ok(msg) => if sender.send(msg).is_err() {
                                            break;
                                        },
                                        Err(err) => eprintln!("[mist] Error deserializing data from subprocess: {}", err)
                                    }
                                },
                                Err(err) => if err.kind() != std::io::ErrorKind::UnexpectedEof {
                                    eprintln!("[mist] Error reading stdin from subprocess: {}", err);
                                    break;
                                },
                            }
                        }

                    });

                    MistClient {
                        callbacks: std::collections::VecDeque::new(),
                        write,
                        receiver,
                        _read: std::marker::PhantomData,
                    }
                }

                pub fn write_data<D: serde::Serialize>(&mut self, data: &D) -> Result<()> {
                    let mut data = bincode::serialize(data)?;
                    let mut payload = (data.len() as u32).to_le_bytes().to_vec();
                    payload.append(&mut data);
                    self.write.write_all(&payload)?;
                    self.write.flush()?;
                    Ok(())
                }

                pub fn poll(&mut self) -> Result<(), Error> {
                    while let Ok(data) = self.receiver.try_recv() {
                        match data {
                            MistServiceToLibrary::Initialized => unreachable!(),
                            MistServiceToLibrary::InitError(_) => unreachable!(),
                            MistServiceToLibrary::Callback(callback) => {
                                self.callbacks.push_back(callback);
                            },
                            MistServiceToLibrary::Result(_) => {}
                        }
                    }

                    Ok(())
                }

                pub fn callbacks(&mut self) -> &mut std::collections::VecDeque<crate::callbacks::MistCallback> {
                    &mut self.callbacks
                }

                $(
                    pub fn [< $module:snake >](&mut self) -> &mut dyn [<MistClient $module>] {
                        self
                    }
                )*
            }

            $(
                impl <R: Read + Send + 'static, W: Write> [<MistClient $module>] for MistClient<R, W> {
                    $(

                            fn $call_name(&mut self, $($arg : $arg_ty),*) -> Result<mist_service!(__fallback_ty$(,$return_ty)?), Error> {
                                let msg = MistLibraryToService::$call_name($($arg),*);
                                if let Err(err) = self.write_data(&msg) {
                                    mist_log_error!(&format!("Error writing data to subprocess: {}", err));
                                    return Err(Error::Mist(MistError::SubprocessLost));
                                }

                                while let Ok(data) = self.receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                                    match data {
                                        MistServiceToLibrary::Initialized => unreachable!(),
                                        MistServiceToLibrary::InitError(_) => unreachable!(),
                                        MistServiceToLibrary::Callback(callback) => {
                                            self.callbacks.push_back(callback);
                                        },
                                        MistServiceToLibrary::Result(Ok(mist_service!{__fallback_ty_ret, $call_name, res $(,$return_ty)?})) => {
                                            $(
                                                let res: $return_ty = res;
                                                return Ok(res);
                                            )?
                                            mist_service!{__fallback_ty_ret$(,$return_ty)?}
                                        },
                                        MistServiceToLibrary::Result(Err(err)) => {
                                            return Err(err);
                                        },
                                        MistServiceToLibrary::Result(_) => {}
                                    }
                                }

                                mist_log_error!("Timeout calling function");
                                return Err(Error::Mist(MistError::Timeout));
                            }
                        )*
                }
            )+

            #[allow(dead_code)]
            #[cfg(feature = "steamworks")]
            pub struct MistServer<S: MistService, R: Read, W: Write>
            {
                service: S,
                write: W,
                receiver: std::sync::mpsc::Receiver<MistLibraryToService>,
                _read: std::marker::PhantomData<R>,
            }

            #[allow(dead_code)]
            #[cfg(feature = "steamworks")]
            impl<S: MistService, R: Read + Send + 'static, W: Write> MistServer<S, R, W> {
                pub fn create(service: S, mut read: R, write: W) -> MistServer<S, R, W> {
                    // stdin reading is blocking, therefore we have a dedicated thread for it. It will always idle while waiting
                    let (sender, receiver) = std::sync::mpsc::channel::<MistLibraryToService>();
                    std::thread::spawn(move || {
                        loop {
                            let mut len_buf = [0u8; 32 / 8];
                            match read.read_exact(&mut len_buf) {
                                Ok(_) => {

                                    let len = u32::from_le_bytes(len_buf) as usize;
                                    let mut msg_buf = vec![0; len];
                                    if let Err(err) = read.read_exact(&mut msg_buf) {
                                        eprintln!("[mist] Error reading mist error call: {}", err);
                                        continue;
                                    }

                                    match bincode::deserialize(&msg_buf) {
                                        Ok(msg) => sender.send(msg).expect("Error sending message to main thread"),
                                        Err(err) => {
                                            eprintln!("[mist] Error parsing bincode in subprocess: {}", err);
                                            continue;
                                        }
                                    }
                                },
                                Err(err) => if err.kind() != std::io::ErrorKind::UnexpectedEof {
                                    eprintln!("[mist] Error reading stdin in subprocess: {}", err);
                                    std::process::exit(1);
                                },
                            }
                        }
                    });

                    MistServer {
                        service,
                        write,
                        receiver,
                        _read: std::marker::PhantomData,
                    }
                }

                pub fn service(&mut self) -> &mut S {
                    &mut self.service
                }

                pub fn write_data<D: serde::Serialize>(&mut self, data: &D) -> Result<()> {
                    let mut data = bincode::serialize(data)?;
                    let mut payload = (data.len() as u32).to_le_bytes().to_vec();
                    payload.append(&mut data);
                    self.write.write(&payload)?;
                    self.write.flush()?;
                    Ok(())
                }

                pub fn recv_timeout(&mut self, mut timeout: Duration) {
                    loop {
                        match self.receiver.recv_timeout(timeout) {
                            Ok(msg) => {
                                match msg {
                                    $($(
                                        MistLibraryToService::$call_name($($arg),*) => {
                                            #[allow(unused_variables)]
                                            let ret = self.service.$call_name($($arg),*);


                                                // Use the $return_ty so we can ensure this is a function which has a return value
                                                let ret: Result<mist_service!(__fallback_ty$(,$return_ty)?), Error> = ret;
                                                let msg = MistServiceToLibrary::Result(match ret {
                                                    #[allow(unused_variables)] // This is unused for functions which return an unit
                                                    Ok(res) => Ok(mist_service!{__fallback_ty_ret, $call_name, res $(,$return_ty)?}),
                                                    Err(err) => Err(err)
                                                }); //(ret)));
                                                if let Err(err) = self.write_data(&msg) {
                                                    eprintln!("[mist] Error replying to library call in subprocess: {}", err);
                                                }
                                        }
                                    )*)*
                                }

                                // Keep timeout zero for subsequent polls so we stop when there is no more calls
                                timeout = Duration::default();
                            },
                            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => break,
                            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                                eprintln!("[mist] Disconnected from stdin channel in subprocess");
                                std::process::exit(1);
                            },
                        }
                    }
                }
            }


            // Enums used to serialize messages with bincode
            #[allow(non_camel_case_types)]
            #[derive(Serialize, Deserialize, PartialEq)]
            enum MistLibraryToService {
                $($(
                    $call_name($($arg_ty),*)
                ),*),*
            }

            #[allow(non_camel_case_types)]
            #[derive(Serialize, Deserialize, PartialEq)]
            pub enum MistServiceToLibraryResult {
                $($(
                    $call_name $( ($return_ty) )?
                ),*),*
            }

            #[derive(Serialize, Deserialize, PartialEq)]
            pub enum MistServiceToLibrary {
                Initialized,
                InitError(String),
                Callback(crate::callbacks::MistCallback),
                Result(Result<MistServiceToLibraryResult, Error>)
            }
        }
    }
}

macro_rules! mist_errors {
    (__format, $out:expr, $kind:ident, $err:ident, $err_code:expr) => {
        $out.push_str(&format!("{}Error_{} = {}", stringify!($kind), stringify!($err), $err_code));
    };
    (__format, $out:expr, $kind:ident, $err:ident) => {
        $out.push_str(&format!("{}Error_{}", stringify!($kind), stringify!($err)));
    };
    ($($kind:ident: $code:literal { $($err:ident $(= $err_code:literal)*),* }),*) => {
        paste::paste! {
            #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
            pub enum Error {
                $($kind([<$kind Error>])),+
            }

            #[repr(u16)]
            pub enum MistResultCode {
                #[allow(dead_code)]
                Success = 0,
                $($kind = $code),+
            }

            $(
                #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
                #[repr(u16)]
                pub enum [<$kind Error>] {
                    $(
                        $err$( = $err_code)*
                    ),*
                }
            )*


            impl From<Error> for MistResult {
                fn from(err: Error) -> Self {
                    match err {
                        $(
                            Error::$kind(err) => (MistResultCode::$kind as MistResult | (err as MistResult) << 16)
                        ),*
                    }
                }
            }

            // Needed for subprocess unwrap
            impl From<Error> for std::result::Result<(), Error> {
                fn from(err: Error) -> Self {
                    Err(err)
                }
            }

            // This will get LTO'd on normal builds so it should be fine
            // Called from generate headers tool
            #[allow(dead_code)]
            pub fn generate_header() -> String {
                let mut out: String = "enum {\n\tMistResult_Success = 0".into();

                $(
                    out.push_str(&format!(",\n\tMistResult_{} = {}", stringify!($kind), $code));
                )*

                out.push_str("\n};\n\n");

                $(
                    out.push_str("enum {\n\t");
                    let mut first = true;

                    $(
                        #[allow(unused_assignments)]
                        if first {
                            first = false;
                        } else {
                            out.push_str(",\n\t");
                        }

                        mist_errors!(__format, out, $kind, $err $(,$err_code)*);
                    )*

                    out.push_str("\n};\n\n");
                )*

                out.pop(); // Remove last newline

                out
            }
        }
    }
}

macro_rules! mist_callbacks {
    ($($module:ident {
        $($callback_ident:ident {
            $($callback_var_ident:ident => $callback_field_ident:ident: $callback_var_ty:ty),*
        }),*
    }),*) => {
        paste::paste! {
            use serde_derive::{Serialize, Deserialize};

            $(
                $(
                    #[derive(Serialize, Deserialize, PartialEq)]
                    #[repr(C)]
                    pub struct [<MistCallback $module $callback_ident>] {
                        $($callback_field_ident: $callback_var_ty),*
                    }
                )*
            )*

            #[derive(Serialize, Deserialize, PartialEq)]
            pub struct MistCallback {
                pub user: SteamUser,
                pub callback: u32,
                pub data: MistCallbacks
            }

            #[derive(Serialize, Deserialize, PartialEq)]
            pub enum MistCallbacks {
                $($(
                    [<$module $callback_ident>] ([<MistCallback $module $callback_ident>])
                ),*),*
            }

            #[cfg(feature = "steamworks")]
            impl MistCallback {
                #[allow(dead_code)] // It is actually used, no idea why rust-analyzer thinks otherwise
                pub fn from_steam_callback(user: SteamUser, callback: &steamworks_sys::CallbackMsg_t) -> Option<MistCallback> {
                    let callback_id = callback.m_iCallback as u32;
                    match callback_id {
                        $(
                            $(
                               steamworks_sys::[<$callback_ident _t_k_iCallback>] => {
                                    let data_ptr: *const steamworks_sys::[<$callback_ident _t>] = callback.m_pubParam as *const _;
                                    #[allow(unused_variables)] // This can be unused if the struct has no fields
                                    let data = unsafe { &*data_ptr };

                                    Some(MistCallback {
                                        user,
                                        callback: callback_id,
                                        data: MistCallbacks::[<$module $callback_ident>] ([<MistCallback $module $callback_ident>] {
                                            $($callback_field_ident: data.$callback_var_ident),*
                                        })
                                    })
                               },
                            ),*
                        )*
                        _ => None
                    }
                }
            }
        }
    }
}
