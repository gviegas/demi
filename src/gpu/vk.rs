// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_char;
use std::ptr;

use vk_sys::{
    ApplicationInfo, Instance, InstanceCreateInfo, InstanceFp, API_VERSION_1_3,
    STRUCTURE_TYPE_APPLICATION_INFO, STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS,
};

use crate::gpu::Gpu;

/// Initializes the `vk_sys` back-end.
pub fn init() -> Option<Box<dyn Gpu>> {
    match vk_sys::init() {
        Ok(_) => {
            let inst_vers = check_version()?;
            let inst = create_instance()?;
            let inst_fp = match unsafe { InstanceFp::new(inst) } {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("[!] could not load instance functions ({})", e);
                    return None;
                }
            };
            // TODO
            Some(Box::new(Impl {
                inst,
                inst_fp,
                inst_vers,
            }))
        }
        Err(e) => {
            eprintln!("[!] gpu::vk: could not initialize library ({})", e);
            None
        }
    }
}

/// Checks whether the instance version is adequate (i.e., not a variant).
/// Returns the raw version.
fn check_version() -> Option<u32> {
    let mut vers = 0;
    let res = unsafe { vk_sys::enumerate_instance_version(&mut vers) };
    if res == SUCCESS {
        match vk_sys::api_version_variant(vers) {
            0 => {
                println!(
                    "Using Vulkan v{}.{}.{}",
                    vk_sys::api_version_major(vers),
                    vk_sys::api_version_minor(vers),
                    vk_sys::api_version_patch(vers)
                );
                Some(vers)
            }
            x => {
                eprintln!("[!] gpu::vk: implementation is a variant (#{})", x);
                None
            }
        }
    } else {
        eprintln!("[!] gpu::vk: could not check version ({})", res);
        None
    }
}

/// Creates a new instance.
fn create_instance() -> Option<Instance> {
    const NAME: *const c_char = b"demi\0" as *const u8 as *const _;
    const VERS: u32 = 1;

    let app_info = ApplicationInfo {
        s_type: STRUCTURE_TYPE_APPLICATION_INFO,
        next: ptr::null(),
        application_name: ptr::null(), // TODO
        application_version: 0,        // TODO
        engine_name: NAME,
        engine_version: VERS,
        api_version: API_VERSION_1_3,
    };

    const EXTS: &[*const c_char; 2] = if cfg!(target_os = "linux") {
        &[
            b"VK_KHR_surface\0" as *const u8 as *const _,
            b"VK_KHR_wayland_surface\0" as *const u8 as *const _,
        ]
    } else if cfg!(windows) {
        &[
            b"VK_KHR_surface\0" as *const u8 as *const _,
            b"VK_KHR_win32_surface\0" as *const u8 as *const _,
        ]
    } else {
        unreachable!();
    };

    let info = InstanceCreateInfo {
        s_type: STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        next: ptr::null(),
        flags: 0,
        application_info: &app_info,
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: EXTS.len() as _,
        enabled_extension_names: EXTS as *const _,
    };

    let mut inst = ptr::null_mut();
    let res = unsafe { vk_sys::create_instance(&info, ptr::null(), &mut inst) };
    match res {
        SUCCESS => {
            if !inst.is_null() {
                Some(inst)
            } else {
                eprintln!("[!] gpu::vk: unexpected null instance");
                None
            }
        }
        x => {
            eprintln!("[!] gpu::vk: could not create instance ({})", x);
            None
        }
    }
}

/// `Gpu` implementation using `vk_sys` as back-end.
#[derive(Debug)]
pub struct Impl {
    inst: Instance,
    inst_fp: InstanceFp,
    inst_vers: u32,
}

impl Gpu for Impl {}

impl Drop for Impl {
    fn drop(&mut self) {
        // TODO
        unsafe {
            // NOTE: This call invalidates `self.inst_fp`.
            self.inst_fp.destroy_instance(self.inst, ptr::null());
        }
        vk_sys::fini();
    }
}
