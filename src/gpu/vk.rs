// Copyright 2022 Gustavo C. Viegas. All rights reserved.

use std::ffi::{c_char, CStr};
use std::mem;
use std::ptr;

use vk_sys::{
    ApplicationInfo, Device, DeviceCreateInfo, DeviceFp, DeviceQueueCreateInfo, Instance,
    InstanceCreateInfo, InstanceFp, PhysicalDevice, PhysicalDeviceFeatures,
    PhysicalDeviceProperties, Queue, QueueFlags, API_VERSION_1_3, FALSE,
    PHYSICAL_DEVICE_TYPE_DISCRETE_GPU, PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU, QUEUE_COMPUTE_BIT,
    QUEUE_GRAPHICS_BIT, STRUCTURE_TYPE_APPLICATION_INFO, STRUCTURE_TYPE_DEVICE_CREATE_INFO,
    STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO, STRUCTURE_TYPE_INSTANCE_CREATE_INFO, SUCCESS, TRUE,
};

use crate::gpu::Gpu;

/// Initializes the `vk_sys` back-end.
pub fn init() -> Option<Box<dyn Gpu>> {
    match vk_sys::init() {
        Ok(_) => {
            let inst_vers = check_instance_version()?;
            let inst = create_instance()?;
            let inst_fp = match unsafe { InstanceFp::new(inst) } {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("[!] could not load instance functions ({})", e);
                    return None;
                }
            };
            let phys_dev = select_device(inst, &inst_fp)?;
            let dev = create_device(phys_dev, &inst_fp)?;
            let dev_fp = match unsafe { DeviceFp::new(dev, &inst_fp) } {
                Ok(x) => x,
                Err(e) => {
                    eprintln!("[!] could not load device functions ({})", e);
                    return None;
                }
            };
            let fam_idx = check_device_queue(phys_dev, &inst_fp).unwrap();
            let queue = (first_queue(fam_idx, dev, &dev_fp), fam_idx);
            // TODO
            Some(Box::new(Impl {
                inst,
                inst_fp,
                inst_vers,
                phys_dev,
                dev,
                dev_fp,
                queue,
            }))
        }
        Err(e) => {
            eprintln!("[!] gpu::vk: could not initialize library ({})", e);
            None
        }
    }
}

/// Checks whether the instance version is adequate (i.e., not a variant).
///
/// Returns the raw version on success.
fn check_instance_version() -> Option<u32> {
    let mut vers = 0;
    let res = unsafe { vk_sys::enumerate_instance_version(&mut vers) };
    if res == SUCCESS {
        match vk_sys::api_version_variant(vers) {
            0 => {
                println!(
                    "gpu::vk: instance version is {}.{}.{}",
                    vk_sys::api_version_major(vers),
                    vk_sys::api_version_minor(vers),
                    vk_sys::api_version_patch(vers)
                );
                Some(vers)
            }
            other => {
                eprintln!("[!] gpu::vk: instance is a variant ({})", other);
                None
            }
        }
    } else {
        eprintln!("[!] gpu::vk: could not check version ({})", res);
        None
    }
}

#[cfg(target_os = "linux")]
const INSTANCE_EXTS: &[*const c_char; 2] = &[
    b"VK_KHR_surface\0" as *const u8 as _,
    b"VK_KHR_wayland_surface\0" as *const u8 as _,
];

#[cfg(windows)]
const INSTANCE_EXTS: &[*const c_char; 2] = &[
    b"VK_KHR_surface\0" as *const u8 as _,
    b"VK_KHR_win32_surface\0" as *const u8 as _,
];

/// Checks whether the instance has all required extensions.
fn instance_has_extensions() -> bool {
    let mut count = 0;
    let res = unsafe {
        vk_sys::enumerate_instance_extension_properties(ptr::null(), &mut count, ptr::null_mut())
    };
    if res != SUCCESS {
        eprintln!(
            "[!] gpu::vk: could not enumerate instance extensions ({})",
            res
        );
        return false;
    }
    unsafe {
        let mut props = Vec::with_capacity(count as _);
        match vk_sys::enumerate_instance_extension_properties(
            ptr::null(),
            &mut count,
            props.as_mut_ptr(),
        ) {
            SUCCESS => {
                props.set_len(count as _);
                assert!(props.iter().all(|x| x.extension_name.last() == Some(&0)));
                'outer: for i in INSTANCE_EXTS {
                    let ext = CStr::from_ptr(i.cast());
                    for j in &props {
                        if ext == CStr::from_ptr(&j.extension_name as _) {
                            continue 'outer;
                        }
                    }
                    eprintln!("[!] gpu::vk: instance does not support {:?}", ext);
                    return false;
                }
                true
            }
            other => {
                eprintln!(
                    "[!] gpu::vk: could not enumerate instance extensions ({})",
                    other
                );
                false
            }
        }
    }
}

/// Creates a new instance.
fn create_instance() -> Option<Instance> {
    const NAME: *const c_char = b"demi\0" as *const u8 as _;
    const VERS: u32 = 1;

    if !instance_has_extensions() {
        return None;
    }

    let info = InstanceCreateInfo {
        s_type: STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        next: ptr::null(),
        flags: 0,
        application_info: &ApplicationInfo {
            s_type: STRUCTURE_TYPE_APPLICATION_INFO,
            next: ptr::null(),
            application_name: ptr::null(), // TODO
            application_version: 0,        // TODO
            engine_name: NAME,
            engine_version: VERS,
            api_version: API_VERSION_1_3,
        },
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: INSTANCE_EXTS.len() as _,
        enabled_extension_names: INSTANCE_EXTS as _,
    };

    let mut inst = ptr::null_mut();
    match unsafe { vk_sys::create_instance(&info, ptr::null(), &mut inst) } {
        SUCCESS => {
            if !inst.is_null() {
                Some(inst)
            } else {
                eprintln!("[!] gpu::vk: unexpected null instance");
                None
            }
        }
        other => {
            eprintln!("[!] gpu::vk: could not create instance ({})", other);
            None
        }
    }
}

/// Selects a physical device.
fn select_device(inst: Instance, fp: &InstanceFp) -> Option<PhysicalDevice> {
    let mut count = 0;
    let res = unsafe { fp.enumerate_physical_devices(inst, &mut count, ptr::null_mut()) };
    if res != SUCCESS {
        eprintln!("[!] gpu::vk: could not enumerate devices ({})", res);
        return None;
    }
    let mut devs = Vec::with_capacity(count as _);
    unsafe {
        match fp.enumerate_physical_devices(inst, &mut count, devs.as_mut_ptr()) {
            SUCCESS => devs.set_len(count as _),
            other => {
                eprintln!("[!] gpu::vk: could not enumerate devices ({})", other);
                return None;
            }
        }
    }
    unsafe {
        let mut dev_prop = mem::zeroed();
        let mut dev = ptr::null_mut();
        for i in devs {
            fp.get_physical_device_properties(i, &mut dev_prop);
            if check_device_version(i, Some(&dev_prop), None).is_none()
                || check_device_queue(i, fp).is_none()
                || !device_has_features(i, fp)
                || !device_has_extensions(i, fp)
            {
                continue;
            }
            match dev_prop.device_type {
                PHYSICAL_DEVICE_TYPE_DISCRETE_GPU => {
                    // This one will do.
                    dev = i;
                    break;
                }
                PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU if dev.is_null() => {
                    // Choose this one for now - we may yet find a
                    // discrete device.
                    dev = i;
                }
                // Ignore the rest.
                // TODO: What about virtual GPU? and `other`?
                // Should be fine to ignore CPU type though.
                _ => (),
            }
        }
        if !dev.is_null() {
            println!(
                "gpu::vk: chose device {:?}",
                device_name(dev, None, Some(fp))
            );
            Some(dev)
        } else {
            eprintln!("[!] gpu::vk: could not find a suitable device");
            None
        }
    }
}

/// Checks whether the device version is adequate (i.e., not a variant).
///
/// Either `prop` or `fp` must be a `Some` variant.
///
/// Returns the raw version on success.
fn check_device_version(
    dev: PhysicalDevice,
    prop: Option<&PhysicalDeviceProperties>,
    fp: Option<&InstanceFp>,
) -> Option<u32> {
    let check =
        |prop: &PhysicalDeviceProperties| match vk_sys::api_version_variant(prop.api_version) {
            0 => {
                println!(
                    "gpu::vk: {:?} version is {}.{}.{}",
                    device_name(dev, Some(prop), None),
                    vk_sys::api_version_major(prop.api_version),
                    vk_sys::api_version_minor(prop.api_version),
                    vk_sys::api_version_patch(prop.api_version)
                );
                Some(prop.api_version)
            }
            other => {
                eprintln!(
                    "[!] gpu::vk: {:?} is a variant ({})",
                    device_name(dev, Some(prop), None),
                    other
                );
                None
            }
        };

    if let Some(x) = prop {
        check(x)
    } else if let Some(x) = fp {
        unsafe {
            let mut prop = mem::zeroed();
            x.get_physical_device_properties(dev, &mut prop);
            check(&prop)
        }
    } else {
        unreachable!();
    }
}

/// Checks whether a given physical device has at least one queue that
/// can be used for graphics.
///
/// Returns the queue family index on success.
fn check_device_queue(dev: PhysicalDevice, fp: &InstanceFp) -> Option<u32> {
    let mut count = 0;
    unsafe {
        fp.get_physical_device_queue_family_properties(dev, &mut count, ptr::null_mut());
    }
    let mut props = Vec::with_capacity(count as _);
    unsafe {
        fp.get_physical_device_queue_family_properties(dev, &mut count, props.as_mut_ptr());
        props.set_len(count as _);
    }
    const FLAGS: QueueFlags = QUEUE_GRAPHICS_BIT | QUEUE_COMPUTE_BIT;
    props
        .into_iter()
        .position(|p| p.queue_flags & FLAGS == FLAGS)
        .map(|i| i as _)
}

/// Checks whether a given physical device has all required features.
fn device_has_features(dev: PhysicalDevice, fp: &InstanceFp) -> bool {
    let mut feat = unsafe { mem::zeroed() };
    unsafe {
        fp.get_physical_device_features(dev, &mut feat);
    }
    // Dynamically uniform.
    if feat.shader_uniform_buffer_array_dynamic_indexing == FALSE {
        eprintln!(
            "[!] gpu::vk: {:?} does not support dynamic indexing of uniform buffers",
            device_name(dev, None, Some(fp))
        );
        return false;
    }
    if feat.shader_sampled_image_array_dynamic_indexing == FALSE {
        eprintln!(
            "[!] gpu::vk: {:?} does not support dynamic indexing of sampled images",
            device_name(dev, None, Some(fp))
        );
        return false;
    }
    if feat.shader_storage_buffer_array_dynamic_indexing == FALSE {
        eprintln!(
            "[!] gpu::vk: {:?} does not support dynamic indexing of storage buffers",
            device_name(dev, None, Some(fp))
        );
        return false;
    }
    if feat.shader_storage_image_array_dynamic_indexing == FALSE {
        eprintln!(
            "[!] gpu::vk: {:?} does not support dynamic indexing of storage images",
            device_name(dev, None, Some(fp))
        );
        return false;
    }
    true
}

const DEVICE_EXTS: &[*const c_char; 1] = &[b"VK_KHR_swapchain\0" as *const u8 as _];

/// Checks whether a given physical device has all required extensions.
fn device_has_extensions(dev: PhysicalDevice, fp: &InstanceFp) -> bool {
    let mut count = 0;
    let res = unsafe {
        fp.enumerate_device_extension_properties(dev, ptr::null(), &mut count, ptr::null_mut())
    };
    if res != SUCCESS {
        eprintln!(
            "[!] gpu::vk: could not enumerate device extensions ({})",
            res
        );
        return false;
    }
    unsafe {
        let mut props = Vec::with_capacity(count as _);
        match fp.enumerate_device_extension_properties(
            dev,
            ptr::null(),
            &mut count,
            props.as_mut_ptr(),
        ) {
            SUCCESS => {
                props.set_len(count as _);
                assert!(props.iter().all(|x| x.extension_name.last() == Some(&0)));
                'outer: for i in DEVICE_EXTS {
                    let ext = CStr::from_ptr(i.cast());
                    for j in &props {
                        if ext == CStr::from_ptr(&j.extension_name as _) {
                            continue 'outer;
                        }
                    }
                    eprintln!(
                        "[!] gpu::vk: {:?} does not support {:?}",
                        device_name(dev, None, Some(fp)),
                        ext
                    );
                    return false;
                }
                true
            }
            other => {
                eprintln!(
                    "[!] gpu::vk: could not enumerate device extensions ({})",
                    other
                );
                false
            }
        }
    }
}

/// Returns the name of a given device.
///
/// Either `prop` or `fp` must be a `Some` variant.
fn device_name(
    dev: PhysicalDevice,
    prop: Option<&PhysicalDeviceProperties>,
    fp: Option<&InstanceFp>,
) -> String {
    let to_str = |prop: &PhysicalDeviceProperties| unsafe {
        match CStr::from_ptr(&prop.device_name as _).to_str() {
            Ok(x) => x.to_string(),
            Err(_) => "<unknown>".to_string(),
        }
    };
    if let Some(x) = prop {
        to_str(x)
    } else if let Some(x) = fp {
        unsafe {
            let mut prop = mem::zeroed();
            x.get_physical_device_properties(dev, &mut prop);
            to_str(&prop)
        }
    } else {
        unreachable!();
    }
}

/// Creates a new device.
fn create_device(phys_dev: PhysicalDevice, fp: &InstanceFp) -> Option<Device> {
    // NOTE: There is no reliable way to know beforehand which
    // queue family can be used for presentation, so we create
    // one queue of every family available.
    // TODO: Skip this on Android (when supported).
    let mut queue_cnt = 0;
    unsafe {
        fp.get_physical_device_queue_family_properties(phys_dev, &mut queue_cnt, ptr::null_mut());
    }
    let mut queue_infos = Vec::with_capacity(queue_cnt as _);
    for i in 0..queue_cnt {
        queue_infos.push(DeviceQueueCreateInfo {
            s_type: STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            queue_family_index: i,
            queue_count: 1,
            queue_priorities: &1f32,
        });
    }

    // TODO: Enable other supported features.
    let mut feat: PhysicalDeviceFeatures = unsafe { mem::zeroed() };
    feat.shader_uniform_buffer_array_dynamic_indexing = TRUE;
    feat.shader_sampled_image_array_dynamic_indexing = TRUE;
    feat.shader_storage_buffer_array_dynamic_indexing = TRUE;
    feat.shader_storage_image_array_dynamic_indexing = TRUE;

    let info = DeviceCreateInfo {
        s_type: STRUCTURE_TYPE_DEVICE_CREATE_INFO,
        next: ptr::null(),
        flags: 0,
        queue_create_info_count: queue_cnt,
        queue_create_infos: queue_infos.as_ptr(),
        enabled_layer_count: 0,
        enabled_layer_names: ptr::null(),
        enabled_extension_count: DEVICE_EXTS.len() as _,
        enabled_extension_names: DEVICE_EXTS as _,
        enabled_features: &feat,
    };

    let mut dev = ptr::null_mut();
    match unsafe { fp.create_device(phys_dev, &info, ptr::null(), &mut dev) } {
        SUCCESS => {
            if !dev.is_null() {
                Some(dev)
            } else {
                eprintln!("[!] gpu::vk: unexpected null device");
                None
            }
        }
        other => {
            eprintln!("[!] gpu::vk: could not create device ({})", other);
            None
        }
    }
}

/// Gets the first queue of a given family.
fn first_queue(fam_idx: u32, dev: Device, fp: &DeviceFp) -> Queue {
    let mut queue = ptr::null_mut();
    unsafe {
        fp.get_device_queue(dev, fam_idx, 0, &mut queue);
    }
    queue
}

/// `Gpu` implementation using `vk_sys` as back-end.
#[derive(Debug)]
pub struct Impl {
    inst: Instance,
    inst_fp: InstanceFp,
    inst_vers: u32,
    phys_dev: PhysicalDevice,
    dev: Device,
    dev_fp: DeviceFp,
    queue: (Queue, u32),
}

impl Gpu for Impl {}

impl Drop for Impl {
    fn drop(&mut self) {
        // TODO
        unsafe {
            // NOTE: This call invalidates `self.dev_fp`.
            self.dev_fp.destroy_device(self.dev, ptr::null());
        }
        unsafe {
            // NOTE: This call invalidates `self.inst_fp`.
            self.inst_fp.destroy_instance(self.inst, ptr::null());
        }
        vk_sys::fini();
    }
}