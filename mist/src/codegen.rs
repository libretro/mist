// I know this macro might look scary, but it abstract away all the painful IPC protocol work
macro_rules! mist_service {
    ($(fn $call_name:ident($($arg:ident : $arg_ty:ty),*)$(-> $return_ty:ty)?;)+) => {
        use anyhow::Result;
        use serde_derive::{Serialize, Deserialize};
        // Some are just used for client, and some just for server
        #[allow(unused_imports)]
        use std::{io::{Read, Write}, time::Duration};

        // This trait is required to be implemented by the subprocess
        pub trait MistService {
            $(
                fn $call_name(&mut self $(, $arg : $arg_ty)*) $(-> $return_ty)?;
            )+
        }

        #[allow(dead_code)]
        #[cfg(not(feature = "steamworks"))]
        pub struct MistClient<R: Read, W: Write> {
            write: W,
            pub receiver: std::sync::mpsc::Receiver<MistServiceToLibrary>,
            _read: std::marker::PhantomData<R>,
        }

        #[allow(dead_code)]
        #[cfg(not(feature = "steamworks"))]
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
                    write,
                    receiver,
                    _read: std::marker::PhantomData,
                }
            }

            pub fn write_data<D: serde::Serialize>(&mut self, data: &D) -> Result<()> {
                let mut data = bincode::serialize(data)?;
                let mut payload = (data.len() as u32).to_le_bytes().to_vec();
                payload.append(&mut data);
                self.write.write(&payload)?;
                self.write.flush()?;
                Ok(())
            }

            $(
                pub fn $call_name(&mut self, $($arg : $arg_ty),*) $(-> Option<$return_ty>)? {
                    // Reset the error
                    crate::mist_set_error("");
                    let msg = MistLibraryToService::$call_name($($arg),*);
                    if let Err(err) = self.write_data(&msg) {
                        crate::mist_set_error(&format!("Error writing data to subprocess: {}", err));
                        $(
                            let _ignore: std::marker::PhantomData<$return_ty> = std::marker::PhantomData;
                            return None;
                        )?
                    }

                    $(
                        while let Ok(data) = self.receiver.recv_timeout(std::time::Duration::from_millis(100)) {
                            match data {
                                MistServiceToLibrary::Result(res) => {
                                    match res {
                                        MistServiceToLibraryResult::$call_name(res) => {
                                            let res: $return_ty = res;
                                            return Some(res);
                                        }
                                        _ => ()
                                    }
                                },
                                // TODO: Add events to a queue for poll
                                _ => ()
                            }
                        }

                        crate::mist_set_error("Timeout calling function");
                        None
                    )?
                }
            )*
        }

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
                                $(
                                    MistLibraryToService::$call_name($($arg),*) => {
                                        #[allow(unused_variables)]
                                        let ret = self.service.$call_name($($arg),*);

                                        $(
                                            // Use the $return_ty so we can ensure this is a function which has a return value
                                            let ret: $return_ty = ret;
                                            let msg = MistServiceToLibrary::Result(MistServiceToLibraryResult::$call_name(ret));
                                            if let Err(err) = self.write_data(&msg) {
                                                eprintln!("[mist] Error replying to library call in subprocess: {}", err);
                                            }
                                        )?
                                    }
                                )*
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
            $(
                $call_name($($arg_ty),*)
            ),*
        }

        #[allow(non_camel_case_types)]
        #[derive(Serialize, Deserialize, PartialEq)]
        pub enum MistServiceToLibraryResult {
            $(
                $call_name($($return_ty)?)
            ),*
        }

        #[derive(Serialize, Deserialize, PartialEq)]
        pub enum MistServiceToLibrary {
            Initialized,
            InitError(String),
            Result(MistServiceToLibraryResult)
        }
    }
}
