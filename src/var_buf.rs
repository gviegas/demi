// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Storage of variable-size data.

use std::cmp::Ordering;
use std::io;
use std::ops::Range;
use std::ptr::{self, NonNull};

use crate::bit_vec::BitVec;

/// [`VarBuf`]'s allocation.
pub trait VarAlloc {
    /// The stride between allocated blocks, in bytes.
    ///
    /// It must be a power of two.
    const STRIDE: usize = 512;

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
    ///
    /// It must be a multiple of `STRIDE`, or `0`.
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
    ptr: NonNull<()>,
    alloc: T,
    bits: BitVec<u32>,
}

impl<T: VarAlloc> VarBuf<T> {
    const BIT_N: usize = 32;

    /// Creates a new [`VarBuf`] using a given allocation.
    pub fn new(mut alloc: T) -> Self {
        let size = alloc.size();
        let (ptr, bits) = if size > 0 {
            if size % (T::STRIDE * Self::BIT_N) != 0 {
                let size = (size + T::STRIDE - 1) & !(T::STRIDE - 1);
                let n = (size / T::STRIDE + Self::BIT_N - 1) / Self::BIT_N;
                let size = n * Self::BIT_N * T::STRIDE;
                if let Ok(ptr) = alloc.grow(size) {
                    (ptr, BitVec::with_count_words(n))
                } else if let Ok(ptr) = alloc.shrink(0) {
                    (ptr, BitVec::new())
                } else {
                    panic!("failed to set VarAlloc");
                }
            } else {
                (
                    alloc.grow(size).unwrap(),
                    BitVec::with_count_words(size / T::STRIDE / Self::BIT_N),
                )
            }
        } else {
            (NonNull::dangling(), BitVec::new())
        };
        Self { ptr, alloc, bits }
    }

    /// Allocates an entry.
    pub fn alloc(&mut self, size: usize) -> io::Result<VarEntry> {
        if size == 0 {
            return Err(io::Error::from(io::ErrorKind::InvalidInput));
        }
        // Enforce alignment at entries' boundaries.
        let size = (size + T::STRIDE - 1) & !(T::STRIDE - 1);
        let n = size / T::STRIDE;

        let idx = if let Some(x) = self.bits.find_contiguous(n) {
            x
        } else {
            let needed_n = (n + Self::BIT_N - 1) & !(Self::BIT_N - 1);
            let needed_size = needed_n * T::STRIDE;
            let cur_n = self.bits.len();
            let cur_size = cur_n * T::STRIDE;

            let min_size = cur_size + needed_size;
            let max_size = std::cmp::max(min_size, cur_size * 2);

            'done: {
                if max_size > min_size {
                    let size = max_size;
                    if let Ok(ptr) = self.alloc.grow(size) {
                        self.ptr = ptr;
                        break 'done self
                            .bits
                            .grow((size - cur_size) / T::STRIDE / Self::BIT_N)
                            .unwrap();
                    }
                }
                // We either failed to allocate `max_size` bytes,
                // or this value is no greater than `min_size`.
                // In any case, try to allocate the minimum.
                self.ptr = self.alloc.grow(min_size)?;
                self.bits.grow(needed_n / Self::BIT_N).unwrap()
            }
        };

        for i in 0..n {
            self.bits.set(idx + i);
        }
        Ok(VarEntry {
            offset: idx * T::STRIDE,
            size,
        })
    }

    /// Frees a given entry.
    pub fn dealloc(&mut self, entry: VarEntry) {
        let start = entry.offset / T::STRIDE;
        let end = start + entry.size / T::STRIDE;
        for i in start..end {
            self.bits.unset(i);
        }
        // TODO: Shrink the allocation.
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

// TODO: These tests assume that `VarBuf::BIT_N` is equal to 32.
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestAlloc(Vec<u8>);

    impl VarAlloc for TestAlloc {
        const STRIDE: usize = 4;

        fn grow(&mut self, new_size: usize) -> io::Result<NonNull<()>> {
            if new_size > self.0.len() {
                self.0.resize(new_size, 0);
            }
            Ok(NonNull::new(self.0.as_mut_ptr().cast()).unwrap())
        }

        fn shrink(&mut self, new_size: usize) -> io::Result<NonNull<()>> {
            if new_size < self.0.len() {
                self.0.resize(new_size, 0);
            }
            Ok(NonNull::new(self.0.as_mut_ptr().cast()).unwrap())
        }

        fn size(&self) -> usize {
            self.0.len()
        }
    }

    impl<T: VarAlloc> VarBuf<T> {
        fn assert(&self, alloc_size: usize, rem_bits: usize) {
            assert_eq!(alloc_size, self.alloc.size());
            assert_eq!(rem_bits, self.bits.rem());
            assert_eq!(self.alloc.size() / T::STRIDE, self.bits.len());
        }
    }

    impl VarEntry {
        fn assert(&self, v: &VarBuf<TestAlloc>) {
            assert_eq!(self.offset % TestAlloc::STRIDE, 0);
            assert_eq!(self.size % TestAlloc::STRIDE, 0);
            assert_ne!(self.size, 0);
            assert!(self.range().end <= v.alloc.size());
            let start = self.offset / TestAlloc::STRIDE;
            let end = start + self.size / TestAlloc::STRIDE;
            for i in start..end {
                assert!(v.bits.is_set(i));
            }
        }
    }

    #[test]
    fn new() {
        let v = VarBuf::new(TestAlloc(vec![]));
        v.assert(0, 0);

        let v = VarBuf::new(TestAlloc(vec![0]));
        v.assert(TestAlloc::STRIDE * 32, 32);

        let v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE]));
        v.assert(TestAlloc::STRIDE * 32, 32);

        let v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE * 31]));
        v.assert(TestAlloc::STRIDE * 32, 32);

        let v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE * 32]));
        v.assert(TestAlloc::STRIDE * 32, 32);

        let v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE * 32 + 1]));
        v.assert(TestAlloc::STRIDE * 64, 64)
    }

    #[test]
    fn alloc0() {
        let mut v = VarBuf::new(TestAlloc(vec![]));

        let x = v.alloc(1).unwrap();
        v.assert(TestAlloc::STRIDE * 32, 31);
        x.assert(&v);

        let x = v.alloc(2).unwrap();
        v.assert(TestAlloc::STRIDE * 32, 30);
        x.assert(&v);

        let x = v.alloc(116).unwrap();
        v.assert(TestAlloc::STRIDE * 32, 1);
        x.assert(&v);

        let x = v.alloc(1).unwrap();
        v.assert(TestAlloc::STRIDE * 32, 0);
        x.assert(&v);

        let x = v.alloc(3).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 31);
        x.assert(&v);
    }

    #[test]
    fn alloc() {
        let mut v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE * 2 * 32]));

        let x = v.alloc(2).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 63);
        x.assert(&v);

        let x = v.alloc(1).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 62);
        x.assert(&v);

        let x = v.alloc(20).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 57);
        x.assert(&v);

        let x = v.alloc(100).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 32);
        x.assert(&v);

        let x = v.alloc(4).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 31);
        x.assert(&v);

        let x = v.alloc(3).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 30);
        x.assert(&v);

        let x = v.alloc(25).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 23);
        x.assert(&v);

        let x = v.alloc(24).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 17);
        x.assert(&v);

        let x = v.alloc(26).unwrap();
        v.assert(TestAlloc::STRIDE * 64, 10);
        x.assert(&v);

        let x = v.alloc(41).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 63);
        x.assert(&v);

        let x = v.alloc(4).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 62);
        x.assert(&v);

        let x = v.alloc(10).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 59);
        x.assert(&v);

        let x = v.alloc(200).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 9);
        x.assert(&v);

        let x = v
            .alloc(TestAlloc::STRIDE * VarBuf::<TestAlloc>::BIT_N * 3)
            .unwrap();
        v.assert(TestAlloc::STRIDE * 256, 9 + 32);
        x.assert(&v);

        let x = v.alloc(v.alloc.size()).unwrap();
        v.assert(TestAlloc::STRIDE * 512, 41);
        x.assert(&v);

        let x = v.alloc(v.alloc.size() + 1).unwrap();
        v.assert(TestAlloc::STRIDE * (512 + 512 + 32), 41 + 31);
        x.assert(&v);

        let x = v.alloc(1).unwrap();
        v.assert(TestAlloc::STRIDE * 1056, 71);
        x.assert(&v);

        let x = v.alloc(4).unwrap();
        v.assert(TestAlloc::STRIDE * 1056, 70);
        x.assert(&v);

        let x = v.alloc(15).unwrap();
        v.assert(TestAlloc::STRIDE * 1056, 66);
        x.assert(&v);

        let x = v.alloc(v.alloc.size() - TestAlloc::STRIDE).unwrap();
        v.assert(TestAlloc::STRIDE * (1056 * 2), 67);
        x.assert(&v);

        let x = v.alloc(v.alloc.size() + v.alloc.size() / 2).unwrap();
        v.assert(TestAlloc::STRIDE * (2112 + 2112 + 1056), 67);
        x.assert(&v);

        let x = v.alloc(3).unwrap();
        v.assert(TestAlloc::STRIDE * 5280, 66);
        x.assert(&v);

        let x = v.alloc(16).unwrap();
        v.assert(TestAlloc::STRIDE * 5280, 62);
        x.assert(&v);
    }

    #[test]
    fn dealloc0() {
        let mut v = VarBuf::new(TestAlloc(vec![]));

        let x1 = v.alloc(1).unwrap();
        let x2 = v.alloc(2).unwrap();
        let x3 = v.alloc(116).unwrap();
        let x4 = v.alloc(1).unwrap();
        let x5 = v.alloc(3).unwrap();

        v.assert(TestAlloc::STRIDE * 64, 31);
        v.dealloc(x1);
        v.assert(TestAlloc::STRIDE * 64, 32);
        v.dealloc(x3);
        v.assert(TestAlloc::STRIDE * 64, 61);
        v.dealloc(x5);
        v.assert(TestAlloc::STRIDE * 64, 62);
        v.dealloc(x4);
        v.assert(TestAlloc::STRIDE * 64, 63);
        v.dealloc(x2);
        v.assert(TestAlloc::STRIDE * 64, 64);
    }

    #[test]
    fn dealloc() {
        let mut v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE * 2 * 32]));

        let x1 = v.alloc(2).unwrap();
        let x2 = v.alloc(1).unwrap();
        let x3 = v.alloc(20).unwrap();
        let x4 = v.alloc(100).unwrap();
        let x5 = v.alloc(4).unwrap();
        let x6 = v.alloc(3).unwrap();
        let x7 = v.alloc(25).unwrap();
        let x8 = v.alloc(24).unwrap();
        let x9 = v.alloc(26).unwrap();
        let x10 = v.alloc(41).unwrap();
        let x11 = v.alloc(4).unwrap();
        let x12 = v.alloc(10).unwrap();
        let x13 = v.alloc(200).unwrap();

        v.assert(TestAlloc::STRIDE * 128, 9);
        v.dealloc(x11);
        v.assert(TestAlloc::STRIDE * 128, 10);
        v.dealloc(x3);
        v.assert(TestAlloc::STRIDE * 128, 15);
        v.dealloc(x2);
        v.assert(TestAlloc::STRIDE * 128, 16);
        v.dealloc(x12);
        v.assert(TestAlloc::STRIDE * 128, 19);
        v.dealloc(x13);
        v.assert(TestAlloc::STRIDE * 128, 69);
        v.dealloc(x1);
        v.assert(TestAlloc::STRIDE * 128, 70);
        v.dealloc(x5);
        v.assert(TestAlloc::STRIDE * 128, 71);
        v.dealloc(x9);
        v.assert(TestAlloc::STRIDE * 128, 78);
        v.dealloc(x7);
        v.assert(TestAlloc::STRIDE * 128, 85);
        v.dealloc(x10);
        v.assert(TestAlloc::STRIDE * 128, 96);
        v.dealloc(x4);
        v.assert(TestAlloc::STRIDE * 128, 121);
        v.dealloc(x6);
        v.assert(TestAlloc::STRIDE * 128, 122);
        v.dealloc(x8);
        v.assert(TestAlloc::STRIDE * 128, 128);
    }

    #[test]
    fn alloc_dealloc() {
        let mut v = VarBuf::new(TestAlloc(vec![0; TestAlloc::STRIDE * 2 * 32]));

        let x1 = v.alloc(2).unwrap();
        let x2 = v.alloc(1).unwrap();
        let x3 = v.alloc(20).unwrap();
        let x4 = v.alloc(100).unwrap();
        let x5 = v.alloc(4).unwrap();
        let x6 = v.alloc(3).unwrap();
        let x7 = v.alloc(25).unwrap();
        let x8 = v.alloc(24).unwrap();
        let x9 = v.alloc(26).unwrap();
        let x10 = v.alloc(41).unwrap();
        let x11 = v.alloc(4).unwrap();
        let x12 = v.alloc(10).unwrap();
        let x13 = v.alloc(200).unwrap();

        v.assert(TestAlloc::STRIDE * 128, 9);
        v.dealloc(x11);
        v.assert(TestAlloc::STRIDE * 128, 10);
        v.dealloc(x3);
        v.assert(TestAlloc::STRIDE * 128, 15);
        v.dealloc(x2);
        v.assert(TestAlloc::STRIDE * 128, 16);
        v.dealloc(x12);
        v.assert(TestAlloc::STRIDE * 128, 19);
        v.dealloc(x13);
        v.assert(TestAlloc::STRIDE * 128, 69);

        let x13 = v.alloc(4).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 68);
        v.dealloc(x13);
        v.assert(TestAlloc::STRIDE * 128, 69);
        v.dealloc(x1);
        v.assert(TestAlloc::STRIDE * 128, 70);
        v.dealloc(x5);
        v.assert(TestAlloc::STRIDE * 128, 71);

        let x5 = v.alloc(10).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 68);
        let x1 = v.alloc(6).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 66);

        v.dealloc(x9);
        v.assert(TestAlloc::STRIDE * 128, 73);
        v.dealloc(x7);
        v.assert(TestAlloc::STRIDE * 128, 80);
        v.dealloc(x1);
        v.assert(TestAlloc::STRIDE * 128, 82);
        v.dealloc(x5);
        v.assert(TestAlloc::STRIDE * 128, 85);

        let x1 = v.alloc(21).unwrap();
        v.assert(TestAlloc::STRIDE * 128, 79);

        v.dealloc(x10);
        v.assert(TestAlloc::STRIDE * 128, 90);
        v.dealloc(x4);
        v.assert(TestAlloc::STRIDE * 128, 115);
        v.dealloc(x6);
        v.assert(TestAlloc::STRIDE * 128, 116);
        v.dealloc(x8);
        v.assert(TestAlloc::STRIDE * 128, 122);
        v.dealloc(x1);
        v.assert(TestAlloc::STRIDE * 128, 128);

        let x1 = v.alloc(1024).unwrap();
        v.assert(TestAlloc::STRIDE * (128 + 256), 128);
        let x2 = v.alloc(15).unwrap();
        v.assert(TestAlloc::STRIDE * 384, 124);
        let x3 = v.alloc(4).unwrap();
        v.assert(TestAlloc::STRIDE * 384, 123);

        v.dealloc(x2);
        v.assert(TestAlloc::STRIDE * 384, 127);
        v.dealloc(x1);
        v.assert(TestAlloc::STRIDE * 384, 383);
        v.dealloc(x3);
        v.assert(TestAlloc::STRIDE * 384, 384);
    }
}
