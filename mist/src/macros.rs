// I know this macro might look scary, but it abstract away all the painful IPC protocol work
macro_rules! mist_service {
    ($(fn $call_name:ident($($arg:ident : $arg_ty:ty),*)$(-> $return_ty:ty)?;)+) => {
        use std::io::{Read, Write};

        // This trait is required to be implemented by the subprocess
        pub trait MistService {
            $(
                fn $call_name(&mut self $(, $arg : $arg_ty)*) $(-> $return_ty)?;
            )+
        }

        #[allow(dead_code)]
        pub struct MistClient {

        }

        #[allow(dead_code)]
        pub struct MistServer<S: MistService, R: Read, W: Write>
        {
            service: S,
            write: W,
            _read: std::marker::PhantomData<R>,
        }

        impl<S: MistService, R: Read + Send + 'static, W: Write> MistServer<S, R, W> {
            pub fn create(service: S, mut read: R, write: W) -> MistServer<S, R, W> {
                // stdin reading is blocking, therefore we have a dedicated thread for it. It will always idle while waiting
                std::thread::spawn(move || {
                    let mut buf = Vec::new();
                    while let Ok(_size) = read.read(&mut buf) {
                        // TODO: Parse buf

                        buf.clear();
                    }

                    // TODO: Send error event
                });

                MistServer {
                    service,
                    write,
                    _read: std::marker::PhantomData,
                }
            }

            pub fn service(&mut self) -> &mut S {
                &mut self.service
            }
        }
    }
}
