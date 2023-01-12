// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Storage of variable-size data.

use std::io;
use std::ptr::NonNull;

/// [`VarBuf`]'s allocation.
pub trait VarAlloc {
    /// The minimum alignment supported by the allocation.
    ///
    /// Writes through the allocation's pointer will always
    /// be performed on multiples of this value.
    ///
    /// It must be a power of two, and should be at least
    /// four bytes.
    const MIN_ALIGNMENT: usize = 4;

    /// Grows the allocation to a given size in bytes.
    ///
    /// If `new_size` is less than or equal the current size,
    /// calling this method has no effect and must succeed.
    fn grow(&mut self, new_size: usize) -> io::Result<NonNull<()>>;

    /// Shrinks the allocation to a given size in bytes.
    ///
    /// If `new_size` is greater than or equal the current size,
    /// calling this method has no effect and must succeed.
    fn shrink(&mut self, new_size: usize) -> io::Result<NonNull<()>>;

    /// Returns the size of the allocation, in bytes.
    fn size(&self) -> usize;
}

/// Buffer for storing data of variable size.
#[derive(Debug)]
pub struct VarBuf<T: VarAlloc> {
    // TODO
    alloc: T,
}
