// Copyright 2023 Gustavo C. Viegas. All rights reserved.

use std::ffi::c_void;
use std::io;
use std::mem;
use std::ptr::{self, NonNull};

use vk_sys::{
    Buffer, BufferCreateInfo, DeviceMemory, BUFFER_USAGE_INDEX_BUFFER_BIT,
    BUFFER_USAGE_TRANSFER_DST_BIT, BUFFER_USAGE_TRANSFER_SRC_BIT, BUFFER_USAGE_UNIFORM_BUFFER_BIT,
    BUFFER_USAGE_VERTEX_BUFFER_BIT, ERROR_OUT_OF_DEVICE_MEMORY, ERROR_OUT_OF_HOST_MEMORY,
    SHARING_MODE_EXCLUSIVE, STRUCTURE_TYPE_BUFFER_CREATE_INFO, SUCCESS, WHOLE_SIZE,
};

use crate::gpu::vk::Impl;
use crate::gpu::{BufId, BufOptions, Id};

/// Buffer implementation.
///
/// This type represents a [`BufId`].
#[derive(Debug)]
pub(super) struct BufImpl {
    buf: Buffer,
    mem: DeviceMemory,
    cpu_visible: bool,
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

    /// Creates a new [`BufImpl`] to use as a vertex buffer.
    ///
    /// It supports storage of vertices/indices and copying.
    pub fn new_vb(imp: &Impl, options: &BufOptions) -> io::Result<BufImpl> {
        let info = BufferCreateInfo {
            s_type: STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            size: options.size,
            usage: BUFFER_USAGE_VERTEX_BUFFER_BIT
                | BUFFER_USAGE_INDEX_BUFFER_BIT
                | BUFFER_USAGE_TRANSFER_SRC_BIT
                | BUFFER_USAGE_TRANSFER_DST_BIT,
            sharing_mode: SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            queue_family_indices: ptr::null(),
        };
        let buf = Self::create_buffer(imp, &info)?;
        match Self::bind(imp, buf, options.cpu_visible) {
            Ok(mem) => {
                let data = if options.cpu_visible {
                    // TODO: Consider mapping the memory lazily
                    // and unmapping it when needed.
                    match imp.map(mem, 0, WHOLE_SIZE) {
                        Ok(data) => data,
                        Err(e) => {
                            Self::destroy_buffer(imp, buf);
                            imp.dealloc(mem);
                            return Err(e);
                        }
                    }
                } else {
                    ptr::null_mut()
                };
                Ok(Self {
                    buf,
                    mem,
                    cpu_visible: options.cpu_visible,
                    data,
                })
            }
            Err(e) => {
                Self::destroy_buffer(imp, buf);
                Err(e)
            }
        }
    }

    /// Creates a new [`BufImpl`] to use as an uniform buffer.
    ///
    /// It supports storage of shader uniforms and copying.
    pub fn new_ub(imp: &Impl, options: &BufOptions) -> io::Result<BufImpl> {
        let info = BufferCreateInfo {
            s_type: STRUCTURE_TYPE_BUFFER_CREATE_INFO,
            next: ptr::null(),
            flags: 0,
            size: options.size,
            usage: BUFFER_USAGE_UNIFORM_BUFFER_BIT
                | BUFFER_USAGE_TRANSFER_SRC_BIT
                | BUFFER_USAGE_TRANSFER_DST_BIT,
            sharing_mode: SHARING_MODE_EXCLUSIVE,
            queue_family_index_count: 0,
            queue_family_indices: ptr::null(),
        };
        let buf = Self::create_buffer(imp, &info)?;
        match Self::bind(imp, buf, options.cpu_visible) {
            Ok(mem) => {
                let data = if options.cpu_visible {
                    // TODO: Consider mapping the memory lazily
                    // and unmapping it when needed.
                    match imp.map(mem, 0, WHOLE_SIZE) {
                        Ok(data) => data,
                        Err(e) => {
                            Self::destroy_buffer(imp, buf);
                            imp.dealloc(mem);
                            return Err(e);
                        }
                    }
                } else {
                    ptr::null_mut()
                };
                Ok(Self {
                    buf,
                    mem,
                    cpu_visible: options.cpu_visible,
                    data,
                })
            }
            Err(e) => {
                Self::destroy_buffer(imp, buf);
                Err(e)
            }
        }
    }

    /// Gets a pointer to the underlying memory.
    ///
    /// If the buffer was created as GPU-private, the returned
    /// pointer will be null.
    pub fn data_ptr(&self) -> *mut c_void {
        self.data
    }

    /// Drops the [`BufImpl`].
    pub fn drop_with(self, imp: &Impl) {
        Self::destroy_buffer(imp, self.buf);
        imp.dealloc(self.mem);
    }
}

impl From<BufId> for Box<BufImpl> {
    /// Converts from a [`BufId`] into a boxed [`BufImpl`].
    fn from(buf_id: BufId) -> Self {
        let non_null = match buf_id.0 {
            Id::Ptr(x) => x,
            _ => unreachable!(),
        };
        let raw_ptr = non_null.as_ptr() as *mut BufImpl;
        unsafe { Box::from_raw(raw_ptr) }
    }
}

impl From<&BufId> for &BufImpl {
    /// Converts from a &[`BufId`] into a &[`BufImpl`].
    fn from(buf_id: &BufId) -> Self {
        let non_null = match buf_id.0 {
            Id::Ptr(x) => x,
            _ => unreachable!(),
        };
        unsafe { non_null.cast().as_ref() }
    }
}

impl From<Box<BufImpl>> for BufId {
    /// Converts from a boxed [`BufImpl`] into a [`BufId`].
    fn from(buf_imp: Box<BufImpl>) -> Self {
        let raw_ptr = Box::into_raw(buf_imp) as *mut ();
        let non_null = unsafe { NonNull::new_unchecked(raw_ptr) };
        BufId(Id::Ptr(non_null))
    }
}

#[cfg(test)]
mod tests {
    use crate::gpu::vk::BufImpl;
    use crate::gpu::{self, BufId, BufOptions};

    #[test]
    fn new() {
        crate::init();

        let assert = |buf_imp: &BufImpl| {
            assert!(!vk_sys::is_null_handle(buf_imp.buf));
            assert!(!vk_sys::is_null_handle(buf_imp.mem));
            // NOTE: This assumes that CPU-visible buffer memory
            // is mapped on creation (the current behavior).
            assert!(
                (buf_imp.cpu_visible && !buf_imp.data.is_null())
                    || (!buf_imp.cpu_visible && buf_imp.data.is_null())
            );
        };

        // VB CPU-visible.
        let options = BufOptions {
            size: 16 << 20,
            cpu_visible: true,
        };
        let buf_imp = Box::<BufImpl>::from(gpu::create_vb(&options).unwrap());
        assert(&buf_imp);
        gpu::drop_buffer(&mut BufId::from(buf_imp));

        // VB GPU-private.
        let options = BufOptions {
            size: 64 << 20,
            cpu_visible: false,
        };
        let buf_imp = Box::<BufImpl>::from(gpu::create_vb(&options).unwrap());
        assert(&buf_imp);
        gpu::drop_buffer(&mut BufId::from(buf_imp));

        // UB CPU-visible.
        let options = BufOptions {
            size: 2 << 20,
            cpu_visible: true,
        };
        let buf_imp = Box::<BufImpl>::from(gpu::create_ub(&options).unwrap());
        assert(&buf_imp);
        gpu::drop_buffer(&mut BufId::from(buf_imp));

        // UB GPU-private.
        let options = BufOptions {
            size: 1 << 20,
            cpu_visible: false,
        };
        let buf_imp = Box::<BufImpl>::from(gpu::create_ub(&options).unwrap());
        assert(&buf_imp);
        gpu::drop_buffer(&mut BufId::from(buf_imp));

        crate::shutdown();
    }
}
