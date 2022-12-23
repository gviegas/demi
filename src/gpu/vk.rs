// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use crate::gpu::Gpu;

pub fn init() -> Option<Box<dyn Gpu>> {
    match vk_sys::init() {
        Ok(_) => {
            // TODO
            let (maj, min, pat) = check_version()?;
            println!("[i] Using Vulkan v{}.{}.{}", maj, min, pat);
            Some(Box::new(Impl {}))
        }
        Err(e) => {
            println!("[!] gpu::vk: initialization failed ({})", e);
            None
        }
    }
}

fn check_version() -> Option<(u32, u32, u32)> {
    let mut vers = 0;
    let res = unsafe { vk_sys::enumerate_instance_version(&mut vers) };
    if res == vk_sys::SUCCESS {
        match vk_sys::api_version_variant(vers) {
            0 => Some((
                vk_sys::api_version_major(vers),
                vk_sys::api_version_minor(vers),
                vk_sys::api_version_patch(vers),
            )),
            x => {
                println!("[!] gpu::vk: implementation is a variant (#{})", x);
                None
            }
        }
    } else {
        println!("[!] gpu::vk: could not check version ({})", res);
        None
    }
}

#[derive(Debug)]
pub struct Impl {
    // TODO
}

impl Gpu for Impl {}

impl Drop for Impl {
    fn drop(&mut self) {
        // TODO
        vk_sys::fini();
    }
}
