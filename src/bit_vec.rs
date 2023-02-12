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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;

    impl<T: Unsigned + fmt::Debug> BitVec<T> {
        fn assert(&self, len: usize, rem: usize, elms: &[(usize, T)]) {
            assert_eq!(len, self.len());
            assert_eq!(rem, self.rem());
            for i in elms {
                assert_eq!(self.vec[i.0], i.1);
            }
        }
    }

    #[test]
    fn grow() {
        let mut v = BitVec::<u64>::new();
        v.assert(0, 0, &[]);
        v.grow(1);
        v.assert(64, 64, &[(0, 0)]);
        v.grow(1);
        v.assert(128, 128, &[(0, 0), (1, 0)]);
        v.grow(3);
        v.assert(320, 320, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);

        let mut v: BitVec<u8> = BitVec::new();
        v.assert(0, 0, &[]);
        v.grow(4);
        v.assert(32, 32, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        v.grow(1);
        v.assert(40, 40, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
        v.grow(2);
        v.assert(
            56,
            56,
            &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)],
        );

        let mut v = <BitVec<usize>>::new();
        v.assert(0, 0, &[]);
        v.grow(0);
        v.assert(0, 0, &[]);
        v.grow(1);
        v.assert(usize::BITS as usize, usize::BITS as usize, &[(0, 0)]);
        v.grow(2);
        v.assert(
            usize::BITS as usize * 3,
            usize::BITS as usize * 3,
            &[(0, 0), (1, 0)],
        );
    }

    #[test]
    fn set_unset() {
        let mut v = <BitVec<u16>>::new();
        v.grow(1);
        v.assert(16, 16, &[(0, 0)]);

        v.set(2);
        v.assert(16, 15, &[(0, 0b100)]);

        v.set(10);
        v.assert(16, 14, &[(0, 0b100_0000_0100)]);
        v.set(10);
        v.assert(16, 14, &[(0, 0b100_0000_0100)]);

        v.set(0);
        v.assert(16, 13, &[(0, 0b100_0000_0101)]);
        v.unset(0);
        v.assert(16, 14, &[(0, 0b100_0000_0100)]);
        v.unset(0);
        v.assert(16, 14, &[(0, 0b100_0000_0100)]);

        v.unset(2);
        v.assert(16, 15, &[(0, 0b100_0000_0000)]);

        v.unset(15);
        v.assert(16, 15, &[(0, 0b100_0000_0000)]);
        v.set(15);
        v.assert(16, 14, &[(0, 0b1000_0100_0000_0000)]);

        v.grow(1);
        v.assert(32, 30, &[(0, 0b1000_0100_0000_0000), (1, 0)]);

        v.set(16);
        v.assert(32, 29, &[(0, 0b1000_0100_0000_0000), (1, 0b1)]);
        v.unset(16);
        v.assert(32, 30, &[(0, 0b1000_0100_0000_0000), (1, 0)]);
        v.set(16);
        v.assert(32, 29, &[(0, 0b1000_0100_0000_0000), (1, 0b1)]);

        v.set(17);
        v.assert(32, 28, &[(0, 0b1000_0100_0000_0000), (1, 0b11)]);
        v.set(17);
        v.assert(32, 28, &[(0, 0b1000_0100_0000_0000), (1, 0b11)]);

        v.unset(1);
        v.unset(2);
        v.unset(30);
        v.unset(31);
        v.assert(32, 28, &[(0, 0b1000_0100_0000_0000), (1, 0b11)]);

        v.unset(10);
        v.assert(32, 29, &[(0, 0b1000_0000_0000_0000), (1, 0b11)]);

        v.set(1);
        v.assert(32, 28, &[(0, 0b1000_0000_0000_0010), (1, 0b11)]);

        v.set(31);
        v.assert(
            32,
            27,
            &[(0, 0b1000_0000_0000_0010), (1, 0b1000_0000_0000_0011)],
        );

        v.unset(17);
        v.unset(16);
        v.unset(31);
        v.assert(32, 30, &[(0, 0b1000_0000_0000_0010), (1, 0)]);

        v.unset(1);
        v.unset(15);
        v.assert(32, 32, &[(0, 0), (1, 0)]);

        for i in 0..16 {
            assert!(!v.is_set(i));
            v.set(i);
            assert!(v.is_set(i));
        }
        v.assert(32, 16, &[(0, u16::MAX), (1, 0)]);

        for i in 16..32 {
            assert!(!v.is_set(i));
            v.set(i);
            assert!(v.is_set(i));
        }
        v.assert(32, 0, &[(0, u16::MAX), (1, u16::MAX)]);

        for i in 0..16 {
            assert!(v.is_set(i));
            v.unset(i);
            assert!(!v.is_set(i));
        }
        v.assert(32, 16, &[(0, 0), (1, u16::MAX)]);

        for i in 16..32 {
            assert!(v.is_set(i));
            v.unset(i);
            assert!(!v.is_set(i));
        }
        v.assert(32, 32, &[(0, 0), (1, 0)]);
    }
}
