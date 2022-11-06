// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_char;
use std::sync::Once;

use crate::init::proc::Proc;
use crate::{Device, Instance};

static mut PROC: Option<Proc> = None;

/// Initializes the library.
pub fn init() -> Result<(), &'static str> {
    static INIT: Once = Once::new();
    static mut ERR: Option<Box<String>> = None;
    unsafe {
        INIT.call_once(|| match Proc::new() {
            Ok(x) => PROC = Some(x),
            Err(e) => ERR = Some(Box::new(e)),
        });
        if let Some(ref e) = ERR {
            Err(e)
        } else {
            Ok(())
        }
    }
}

// PFN_vkVoidFunction
type VoidFunction = unsafe extern "C" fn();

// PFN_vkGetInstanceProcAddr
type GetInstanceProcAddr =
    unsafe extern "C" fn(instance: Instance, name: *const c_char) -> Option<VoidFunction>;

// PFN_vkGetDeviceProcAddr
type GetDeviceProcAddr =
    unsafe extern "C" fn(device: Device, name: *const c_char) -> Option<VoidFunction>;

#[cfg(unix)]
mod proc {
    use super::GetInstanceProcAddr;
    use dl::Dl;
    use std::mem;

    pub struct Proc {
        lib: Dl,
        get_instance_proc_addr: GetInstanceProcAddr,
    }

    impl Proc {
        pub fn new() -> Result<Proc, String> {
            const LIB_NAMES: [&str; 2] = ["libvulkan.so.1", "libvulkan.so"];
            let mut err = String::new();
            for i in LIB_NAMES {
                match Dl::new(i, dl::LAZY | dl::LOCAL) {
                    Ok(lib) => match lib.get("vkGetInstanceProcAddr") {
                        Ok(fp) => {
                            return Ok(Proc {
                                lib,
                                get_instance_proc_addr: unsafe { mem::transmute(fp) },
                            })
                        }
                        Err(e) => return Err(e),
                    },
                    Err(e) => err = e,
                }
            }
            Err(err)
        }
    }
}

#[cfg(not(unix))]
compile_error!("not implemented");
