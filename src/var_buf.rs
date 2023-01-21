// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Storage of variable-size data.

use std::cmp::Ordering;
use std::io;
use std::ops::Range;
use std::ptr::{self, NonNull};

/// [`VarBuf`]'s allocation.
pub trait VarAlloc {
    /// The minimum alignment supported by the allocation.
    ///
    /// It must be a power of two.
    const MIN_ALIGN: usize = 4;

    /// Grows the allocation to a given size in bytes.
    ///
    /// If `new_size` is less than or equal to the current size,
    /// calling this method has no effect and must succeed.
    ///
    /// Implementors are allowed to return [`NonNull::dangling`]
    /// when the current size and `new_size` are both zero.
    fn grow(&mut self, new_size: usize) -> io::Result<NonNull<()>>;

    /// Shrinks the allocation to a given size in bytes.
    ///
    /// If `new_size` is greater than or equal to the current size,
    /// calling this method has no effect and must succeed.
    ///
    /// Implementors are allowed to return [`NonNull::dangling`]
    /// when `new_size` is zero.
    fn shrink(&mut self, new_size: usize) -> io::Result<NonNull<()>>;

    /// Returns the size of the allocation, in bytes.
    fn size(&self) -> usize;
}

/// [`VarBuf`]'s data entry.
#[derive(Eq, Debug)]
pub struct VarEntry {
    offset: usize,
    size: usize,
}

impl VarEntry {
    /// Returns its byte offset within the buffer.
    pub fn offset(&self) -> usize {
        self.offset
    }

    /// Returns the number of bytes it occupies.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns `offset..offset + size`.
    pub fn range(&self) -> Range<usize> {
        self.offset..self.offset + self.size
    }
}

impl PartialEq for VarEntry {
    /// Compares the offsets.
    ///
    /// NOTE: This equality is only meaningful when
    /// both entries belong to the same [`VarBuf`].
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

impl PartialOrd for VarEntry {
    /// Compares the offsets.
    ///
    /// NOTE: This ordering is only meaningful when
    /// both entries belong to the same [`VarBuf`].
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for VarEntry {
    /// Compares the offsets.
    ///
    /// NOTE: This ordering is only meaningful when
    /// both entries belong to the same [`VarBuf`].
    fn cmp(&self, other: &Self) -> Ordering {
        self.offset.cmp(&other.offset)
    }
}

/// Buffer for storing data of variable size.
#[derive(Debug)]
pub struct VarBuf<T: VarAlloc> {
    // TODO
    ptr: NonNull<()>,
    alloc: T,
}

impl<T: VarAlloc> VarBuf<T> {
    /// Creates a new [`VarBuf`] using a given allocation.
    pub fn new(mut alloc: T) -> Self {
        Self {
            ptr: alloc.grow(alloc.size()).unwrap(),
            alloc,
        }
    }

    /// Allocates an entry.
    pub fn alloc(&mut self, size: usize) -> io::Result<VarEntry> {
        if size == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }

        // Enforce alignment at entries' boundaries.
        let size = (size + T::MIN_ALIGN - 1) & !(T::MIN_ALIGN - 1);

        // TODO
        if cfg!(test) {
            eprintln!("[!] var_buf: using test alloc");
            let offset = self.alloc.size();
            self.ptr = self.alloc.grow(offset + size)?;
            Ok(VarEntry { offset, size })
        } else {
            todo!();
        }
    }

    /// Frees a given entry.
    pub fn dealloc(&mut self, entry: VarEntry) {
        // TODO
        if cfg!(test) {
            eprintln!("[!] var_buf: using test dealloc");
            if let Ok(x) = self.alloc.shrink(self.alloc.size() - entry.size) {
                self.ptr = x;
            }
        } else {
            todo!();
        }
    }

    /// Copies data to a given entry.
    pub fn copy(&mut self, data: &[u8], entry: &VarEntry) {
        debug_assert_ne!(self.alloc.size(), 0);
        debug_assert!(self.alloc.size() >= entry.offset + entry.size);
        let size = usize::min(entry.size(), data.len());
        unsafe {
            ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.ptr.as_ptr().cast::<u8>().add(entry.offset),
                size,
            );
        }
    }

    /// Copies data to a given entry at a given offset.
    ///
    /// Call `copy` instead when offsetting into `entry`
    /// is not necessary.
    pub fn copy_at(&mut self, data: &[u8], entry: &VarEntry, offset: usize) {
        debug_assert_ne!(self.alloc.size(), 0);
        debug_assert!(self.alloc.size() >= entry.offset + entry.size);
        let offset = usize::min(offset, entry.size);
        let size = usize::min(entry.size - offset, data.len());
        let offset = offset + entry.offset;
        unsafe {
            ptr::copy_nonoverlapping(
                data.as_ptr(),
                self.ptr.as_ptr().cast::<u8>().add(offset),
                size,
            );
        }
    }
}

impl<T: VarAlloc> Drop for VarBuf<T> {
    fn drop(&mut self) {
        // TODO: Likely unnecessary.
        let _ = self.alloc.shrink(0);
    }
}
