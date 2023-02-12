// Copyright 2023 Gustavo C. Viegas. All rights reserved.

//! Vector of bits.

use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Shl, ShlAssign, Shr,
    ShrAssign,
};

pub trait Unsigned:
    Copy
    + Eq
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Shl<usize, Output = Self>
    + ShlAssign<usize>
    + Shr<usize, Output = Self>
    + ShrAssign<usize>
    + Not<Output = Self>
{
    const ZERO: Self;
    const ONE: Self;
    const BITS: usize;
}

macro_rules! def_unsigned {
    ($($t:ty)*) => ($(
        impl Unsigned for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const BITS: usize = Self::BITS as _;
        }
    )*)
}

def_unsigned!(u8 u16 u32 u64 u128 usize);

/// Bit vector type.
#[derive(Debug)]
pub struct BitVec<T: Unsigned> {
    vec: Vec<T>,
    rem: usize,
}

impl<T: Unsigned> BitVec<T> {
    /// Creates an empty bit vector.
    pub fn new() -> Self {
        Self {
            vec: vec![],
            rem: 0,
        }
    }

    /// Increment the vector by `inc` units.
    /// The unit of the increment is `T`. The number of bits
    /// pushed will be equal to `inc * T::BITS`.
    pub fn grow(&mut self, inc: usize) {
        if inc == 1 {
            self.vec.push(T::ZERO);
        } else {
            self.vec.resize(self.vec.len() + inc, T::ZERO);
        }
        self.rem += inc * T::BITS;
    }

    /// Sets a given bit.
    pub fn set(&mut self, bit_idx: usize) {
        let idx = bit_idx / T::BITS;
        let bit = T::ONE << (bit_idx & (T::BITS - 1));
        if self.vec[idx] & bit == T::ZERO {
            self.vec[idx] |= bit;
            self.rem -= 1;
        }
    }

    /// Unsets a given bit.
    pub fn unset(&mut self, bit_idx: usize) {
        let idx = bit_idx / T::BITS;
        let bit = T::ONE << (bit_idx & (T::BITS - 1));
        if self.vec[idx] & bit != T::ZERO {
            self.vec[idx] &= !bit;
            self.rem += 1;
        }
    }

    // Checks whether a given bit is set.
    pub fn is_set(&self, bit_idx: usize) -> bool {
        let idx = bit_idx / T::BITS;
        let bit = T::ONE << (bit_idx & (T::BITS - 1));
        self.vec[idx] & bit == bit
    }

    /// Returns the vector's length in number of bits.
    pub fn len(&self) -> usize {
        self.vec.len() * T::BITS
    }

    /// Returns the number of unset bits.
    pub fn rem(&self) -> usize {
        self.rem
    }

    // TODO...
}
