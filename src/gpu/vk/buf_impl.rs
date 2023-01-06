// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_void;
use std::io;
use std::mem;
use std::ptr;

use vk_sys::{
    Buffer, BufferCreateInfo, DeviceMemory, ERROR_OUT_OF_DEVICE_MEMORY, ERROR_OUT_OF_HOST_MEMORY,
    SUCCESS,
};

use crate::gpu::vk::Impl;
use crate::gpu::{BufId, BufOptions, Id};

/// Buffer implementation.
///
/// This type represents a [`BufferId`].
#[derive(Debug)]
pub(super) struct BufImpl {
    buf: Buffer,
    mem: DeviceMemory,
    data: *mut c_void,
}

impl BufImpl {
    /// Creates a [`vk_sys::Buffer`].
    fn create_buffer(imp: &Impl, info: &BufferCreateInfo) -> io::Result<Buffer> {
        let mut buf = vk_sys::null_handle();
        match unsafe {
            imp.dev_fp
                .create_buffer(imp.dev, info, ptr::null(), &mut buf)
        } {
            SUCCESS => Ok(buf),
            ERROR_OUT_OF_DEVICE_MEMORY | ERROR_OUT_OF_HOST_MEMORY => {
                Err(io::Error::from(io::ErrorKind::OutOfMemory))
            }
            _ => Err(io::Error::from(io::ErrorKind::Other)),
        }
    }

    /// Destroys a [`vk_sys::Buffer`].
    fn destroy_buffer(imp: &Impl, buf: Buffer) {
        unsafe {
            imp.dev_fp.destroy_buffer(imp.dev, buf, ptr::null());
        }
    }

    /// Binds a [`vk_sys::Buffer`] to newly allocated memory.
    ///
    /// NOTE: The memory can only be mapped for host access if
    /// `cpu_visible` is `true`.
    fn bind(imp: &Impl, buf: Buffer, cpu_visible: bool) -> io::Result<DeviceMemory> {
        unsafe {
            let mut req = mem::zeroed();
            imp.dev_fp
                .get_buffer_memory_requirements(imp.dev, buf, &mut req);
            let mem = imp.alloc(&req, cpu_visible)?;
            match imp.dev_fp.bind_buffer_memory(imp.dev, buf, mem, 0) {
                SUCCESS => Ok(mem),
                ERROR_OUT_OF_DEVICE_MEMORY | ERROR_OUT_OF_HOST_MEMORY => {
                    imp.dealloc(mem);
                    Err(io::Error::from(io::ErrorKind::OutOfMemory))
                }
                _ => {
                    imp.dealloc(mem);
                    Err(io::Error::from(io::ErrorKind::Other))
                }
            }
        }
    }
}
