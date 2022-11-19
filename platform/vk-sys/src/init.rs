// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_char;
use std::mem;
use std::ptr;
use std::sync::Once;

use crate::init::proc::Proc;
use crate::{
    CreateInstance, Device, EnumerateInstanceExtensionProperties, EnumerateInstanceLayerProperties,
    EnumerateInstanceVersion, Instance,
};

mod global;
pub use crate::init::global::*;

mod instance;
pub use crate::init::instance::*;

mod device;
pub use crate::init::device::*;

static mut PROC: Option<Proc> = None;
static mut GLOBAL_FP: Option<GlobalFp> = None;

/// Initializes the library.
pub fn init() -> Result<(), &'static str> {
    static INIT: Once = Once::new();
    static mut ERR: String = String::new();
    unsafe {
        INIT.call_once(|| match Proc::new() {
            Ok(proc) => match GlobalFp::new(proc.fp()) {
                Ok(globl) => {
                    PROC = Some(proc);
                    GLOBAL_FP = Some(globl);
                }
                Err(e) => ERR = e,
            },
            Err(e) => ERR = e,
        });
        if PROC.is_some() {
            Ok(())
        } else {
            Err(&ERR)
        }
    }
}

/// Finalizes the library.
pub fn fini() {
    static FINI: Once = Once::new();
    unsafe {
        // Ensure that `drop` is called only once.
        FINI.call_once(|| {
            PROC = None;
            GLOBAL_FP = None;
        });
    }
}

// Global commands.
struct GlobalFp {
    enumerate_instance_layer_properties: EnumerateInstanceLayerProperties,
    enumerate_instance_extension_properties: EnumerateInstanceExtensionProperties,
    create_instance: CreateInstance,

    // v1.1
    enumerate_instance_version: Option<EnumerateInstanceVersion>,
}

impl GlobalFp {
    fn new(get: GetInstanceProcAddr) -> Result<Self, String> {
        macro_rules! get {
            ($bs:expr) => {
                unsafe {
                    match get(ptr::null_mut(), $bs.as_ptr().cast()) {
                        Some(x) => Ok(mem::transmute(x)),
                        None => Err(format!(
                            "could not get FP: {}",
                            String::from_utf8_lossy(&$bs[..$bs.len() - 1])
                        )),
                    }
                }
            };
        }

        Ok(Self {
            enumerate_instance_layer_properties: get!(b"vkEnumerateInstanceLayerProperties\0")?,
            enumerate_instance_extension_properties: get!(
                b"vkEnumerateInstanceExtensionProperties\0"
            )?,
            create_instance: get!(b"vkCreateInstance\0")?,

            enumerate_instance_version: get!(b"vkEnumerateInstanceVersion\0").ok(),
        })
    }
}

/// PFN_vkVoidFunction
pub(crate) type VoidFunction = unsafe extern "C" fn();

/// PFN_vkGetInstanceProcAddr
pub(crate) type GetInstanceProcAddr =
    unsafe extern "C" fn(instance: Instance, name: *const c_char) -> Option<VoidFunction>;

/// PFN_vkGetDeviceProcAddr
pub(crate) type GetDeviceProcAddr =
    unsafe extern "C" fn(device: Device, name: *const c_char) -> Option<VoidFunction>;

#[cfg(unix)]
mod proc {
    use std::mem;

    use crate::GetInstanceProcAddr;
    use dl::Dl;

    pub struct Proc {
        _lib: Dl,
        get_instance_proc_addr: GetInstanceProcAddr,
    }

    impl Proc {
        pub fn new() -> Result<Self, String> {
            const LIB_NAMES: [&str; 2] = ["libvulkan.so.1", "libvulkan.so"];
            let mut err = String::new();
            for i in LIB_NAMES {
                match Dl::new(i, dl::LAZY | dl::LOCAL) {
                    Ok(lib) => match lib.get("vkGetInstanceProcAddr") {
                        Ok(fp) => {
                            return Ok(Self {
                                _lib: lib,
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

        pub fn fp(&self) -> GetInstanceProcAddr {
            self.get_instance_proc_addr
        }
    }
}

#[cfg(not(unix))]
compile_error!("not implemented");