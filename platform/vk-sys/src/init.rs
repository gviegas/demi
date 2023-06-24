use std::ffi::c_char;
use std::hint;
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

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
static RC: AtomicUsize = AtomicUsize::new(0);

/// Initializes the library.
///
/// NOTE: It should be paired with a subsequent [`fini`] call.
pub fn init() -> Result<(), &'static str> {
    static mut ERR: String = String::new();
    match RC.swap(usize::MAX, Ordering::AcqRel) {
        0 => {
            match Proc::new() {
                Ok(proc) => match GlobalFp::new(proc.fp()) {
                    Ok(globl) => unsafe {
                        PROC = Some(proc);
                        GLOBAL_FP = Some(globl);
                    },
                    Err(e) => unsafe { ERR = e },
                },
                Err(e) => unsafe { ERR = e },
            }
            RC.store(1, Ordering::Release);
        }
        usize::MAX => {
            while RC.load(Ordering::Acquire) == usize::MAX {
                hint::spin_loop();
            }
            return init();
        }
        x => {
            assert!(x < isize::MAX as _, "RC overflow");
            RC.store(x + 1, Ordering::Release);
        }
    }
    unsafe {
        if PROC.is_some() {
            Ok(())
        } else {
            Err(&ERR)
        }
    }
}

/// Finalizes the library.
///
/// NOTE: It should be paired with a previous [`init`] call.
pub fn fini() {
    match RC.swap(usize::MAX, Ordering::AcqRel) {
        0 => RC.store(0, Ordering::Release),
        1 => {
            unsafe {
                PROC = None;
                GLOBAL_FP = None;
            }
            RC.store(0, Ordering::Release);
        }
        usize::MAX => {
            while RC.load(Ordering::Acquire) == usize::MAX {
                hint::spin_loop();
            }
            fini();
        }
        x => RC.store(x - 1, Ordering::Release),
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
// TODO
compile_error!("not yet implemented");

#[cfg(test)]
mod tests {
    use std::sync::atomic::Ordering;
    use std::thread;

    use crate::init::{fini, init, GLOBAL_FP, PROC, RC};

    #[test]
    #[ignore]
    // NOTE: This cannot run in parallel with other tests.
    fn crate_init_and_fini() {
        let eq = |x| {
            assert_eq!(x, RC.load(Ordering::Acquire));
            unsafe {
                if x > 0 {
                    assert!(PROC.is_some());
                    assert!(GLOBAL_FP.is_some());
                } else {
                    assert!(PROC.is_none());
                    assert!(GLOBAL_FP.is_none());
                }
            }
        };

        eq(0);
        init().unwrap();
        eq(1);
        fini();
        eq(0);
        init().unwrap();
        init().unwrap();
        eq(2);
        fini();
        eq(1);
        fini();
        eq(0);

        const N: usize = 15;
        let mut join = Vec::with_capacity(N);

        for _ in 0..N {
            join.push(thread::spawn(|| init().unwrap()));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        eq(N);

        for _ in 0..N {
            join.push(thread::spawn(fini));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        eq(0);

        for _ in 0..N {
            join.push(thread::spawn(|| {
                init().unwrap();
                fini();
            }));
        }
        while let Some(x) = join.pop() {
            x.join().unwrap();
        }
        eq(0);
    }
}
