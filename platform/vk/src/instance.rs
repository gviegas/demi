use std::ptr;
use vk_sys as vks;

/// Instance.
pub struct Instance {
    inst: vks::Instance,
    fp: vks::InstanceFp,
}

impl Instance {
    pub fn new(/*_exts: &[&str]*/) -> Result<Self, &'static str> {
        vks::init()?;

        let app_info = vks::ApplicationInfo {
            s_type: vks::STRUCTURE_TYPE_APPLICATION_INFO,
            next: ptr::null(),
            application_name: ptr::null(),
            application_version: 0,
            engine_name: ptr::null(),
            engine_version: 0,
            api_version: vks::API_VERSION_1_0,
        };

        let inst_info = vks::InstanceCreateInfo {
            s_type: vks::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
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
            match vks::create_instance(&inst_info, ptr::null(), &mut inst) {
                vks::SUCCESS => match vks::InstanceFp::new(inst) {
                    Ok(fp) => Ok(Self { inst, fp }),
                    Err(_) => {
                        vks::fini();
                        Err("vk_sys::InstanceFp::new failed")
                    }
                },
                _ => Err("vk_sys::create_instance failed"),
            }
        }
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        unsafe { self.fp.destroy_instance(self.inst, ptr::null()) };
        vks::fini();
    }
}
