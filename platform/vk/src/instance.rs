use std::ptr;

/// Instance.
pub struct Instance {
    inst: vk_sys::Instance,
    fp: vk_sys::InstanceFp,
}

impl Instance {
    pub fn new(/*_exts: &[&str]*/) -> Result<Self, &'static str> {
        vk_sys::init()?;

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
                    Ok(fp) => Ok(Self { inst, fp }),
                    Err(_) => {
                        vk_sys::fini();
                        Err("vk_sys::InstanceFp::new failed")
                    }
                },
                _ => Err("vk_sys::create_instance failed"),
            }
        }
    }
}
