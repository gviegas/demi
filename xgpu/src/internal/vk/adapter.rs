use std::ptr;

use crate::{Error, PowerPreference, Result};

use crate::internal::NAdapter;

pub(super) struct Adapter {
    instance: vk_sys::Instance,
    inst_fp: vk_sys::InstanceFp,
    // TODO
}

impl NAdapter for Adapter {
    // TODO
}

impl Adapter {
    pub(super) fn new(power_pref: PowerPreference) -> Result<Self> {
        if let Err(s) = vk_sys::init() {
            return Err(Error::Internal(s));
        }

        let (instance, inst_fp) = match create_instance() {
            Ok(x) => x,
            Err(e) => {
                vk_sys::fini();
                return Err(e);
            }
        };

        Ok(Self { instance, inst_fp })
    }
}

fn create_instance() -> Result<(vk_sys::Instance, vk_sys::InstanceFp)> {
    let app_info = vk_sys::ApplicationInfo {
        s_type: vk_sys::STRUCTURE_TYPE_APPLICATION_INFO,
        next: ptr::null(),
        application_name: ptr::null(),
        application_version: 0,
        engine_name: ptr::null(),
        engine_version: 0,
        api_version: vk_sys::API_VERSION_1_0,
    };

    let inst_info = vk_sys::InstanceCreateInfo {
        s_type: vk_sys::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        next: ptr::null(),
        flags: 0,
        application_info: &app_info,
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        // TODO
        enabled_extension_count: 0,
        enabled_extension_names: ptr::null(),
    };

    unsafe {
        let mut inst = ptr::null_mut();
        match vk_sys::create_instance(&inst_info, ptr::null(), &mut inst) {
            vk_sys::SUCCESS => match vk_sys::InstanceFp::new(inst) {
                Ok(fp) => Ok((inst, fp)),
                Err(_) => Err(Error::Internal("vk_sys::InstanceFp::new failed")),
            },
            _ => Err(Error::Internal("vk_sys::create_instance failed")),
        }
    }
}
