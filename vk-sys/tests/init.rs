// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ptr;

use vk_sys::{
    self, Device, DeviceCreateInfo, DeviceFp, DeviceQueueCreateInfo, Instance, InstanceCreateInfo,
    InstanceFp,
};

#[test]
fn test_init() {
    vk_sys::init().unwrap();

    let mut version = 0u32;
    assert_eq!(
        unsafe { vk_sys::enumerate_instance_version(&mut version) },
        vk_sys::SUCCESS
    );
    println!(
        "Instance version {}.{}.{} ({})",
        vk_sys::api_version_major(version),
        vk_sys::api_version_minor(version),
        vk_sys::api_version_patch(version),
        vk_sys::api_version_variant(version)
    );

    let instance = create_instance();
    let mut instance_fp = unsafe { InstanceFp::new(instance).unwrap() };
    println!("{instance_fp:#?}");

    let device = create_device(instance, &instance_fp);
    let mut device_fp = unsafe { DeviceFp::new(device, &instance_fp).unwrap() };
    println!("{device_fp:#?}");

    unsafe {
        device_fp.destroy_device(device, ptr::null());
        instance_fp.destroy_instance(instance, ptr::null());
    }
}

fn create_instance() -> Instance {
    let info = InstanceCreateInfo {
        s_type: vk_sys::STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        next: ptr::null(),
        flags: 0,
        application_info: ptr::null(),
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: 0,
        enabled_extension_names: ptr::null(),
    };
    let mut instance = ptr::null_mut();
    unsafe {
        match vk_sys::create_instance(&info, ptr::null(), &mut instance) {
            vk_sys::SUCCESS => (),
            other => panic!("create_instance failed ({})", other),
        }
    }
    instance
}

fn create_device(instance: Instance, instance_fp: &InstanceFp) -> Device {
    unsafe {
        let mut count = 1u32;
        let mut phys_dev = ptr::null_mut();
        match instance_fp.enumerate_physical_devices(instance, &mut count, &mut phys_dev) {
            vk_sys::SUCCESS | vk_sys::INCOMPLETE => (),
            other => panic!("enumerate_physical_devices failed ({})", other),
        }

        let queue_info = DeviceQueueCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            queue_family_index: 0,
            queue_count: 1,
            queue_priorities: &1f32,
        };

        let create_info = DeviceCreateInfo {
            s_type: vk_sys::STRUCTURE_TYPE_DEVICE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            queue_create_info_count: 1,
            queue_create_infos: &queue_info,
            enabled_layer_count: 0,
            enabled_layer_names: ptr::null(),
            enabled_extension_count: 0,
            enabled_extension_names: ptr::null(),
            enabled_features: ptr::null(),
        };

        let mut device = ptr::null_mut();
        match instance_fp.create_device(phys_dev, &create_info, ptr::null(), &mut device) {
            vk_sys::SUCCESS => (),
            other => panic!("create_device failed ({})", other),
        }
        device
    }
}
