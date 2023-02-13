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

    /// Decrements the vector by `dec` units.
    /// The unit of decrement is `T`. The number of bits
    /// popped will be equal to `dec * T::BITS`.
    pub fn shrink(&mut self, dec: usize) {
        match dec {
            0 => (),
            x if x >= self.vec.len() => {
                self.vec.clear();
                self.rem = 0;
            }
            _ => {
                for _ in 0..dec {
                    let mut u = self.vec.pop().unwrap();
                    let mut minus = if u == !T::ZERO {
                        u = T::ZERO;
                        0
                    } else {
                        T::BITS
                    };
                    while u != T::ZERO {
                        if u & T::ONE == T::ONE {
                            minus -= 1;
                        }
                        u >>= 1;
                    }
                    self.rem -= minus;
                }
            }
        }
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

    /// Checks whether a given bit is set.
    pub fn is_set(&self, bit_idx: usize) -> bool {
        let idx = bit_idx / T::BITS;
        let bit = T::ONE << (bit_idx & (T::BITS - 1));
        self.vec[idx] & bit == bit
    }

    /// Searches for an unset bit.
    /// It returns the index of the first unset bit found, or
    /// `None` if every bit in the vector is set.
    /// The bit is not set by this method.
    pub fn find(&self) -> Option<usize> {
        if self.rem == 0 {
            return None;
        }
        self.vec.iter().enumerate().find_map(|(i, &(mut x))| {
            (x != !T::ZERO).then(|| {
                let mut bit = 0;
                while x & T::ONE == T::ONE {
                    bit += 1;
                    x >>= 1;
                }
                i * T::BITS + bit
            })
        })
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

impl<T: Unsigned> Default for BitVec<T> {
    fn default() -> Self {
        Self::new()
    }
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
    fn shrink() {
        let mut v = BitVec::<u32>::new();
        v.grow(1);
        v.assert(32, 32, &[(0, 0)]);
        v.shrink(1);
        v.assert(0, 0, &[]);
        v.shrink(1);
        v.assert(0, 0, &[]);
        v.grow(1);
        v.assert(32, 32, &[(0, 0)]);
        v.shrink(0);
        v.assert(32, 32, &[(0, 0)]);
        v.grow(3);
        v.assert(128, 128, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        v.shrink(0);
        v.assert(128, 128, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        v.shrink(3);
        v.assert(32, 32, &[(0, 0)]);
        v.grow(2);
        v.assert(96, 96, &[(0, 0), (1, 0), (2, 0)]);
        v.shrink(3);
        v.assert(0, 0, &[]);
        v.grow(3);
        v.assert(96, 96, &[(0, 0), (1, 0), (2, 0)]);
        v.shrink(4);
        v.assert(0, 0, &[]);

        v.grow(1);
        v.assert(32, 32, &[(0, 0)]);
        v.set(6);
        v.assert(32, 31, &[(0, 0b100_0000)]);
        v.shrink(1);
        v.assert(0, 0, &[]);
        v.grow(1);
        v.assert(32, 32, &[(0, 0)]);
        v.set(6);
        v.set(0);
        v.set(31);
        v.assert(32, 29, &[(0, 0x80_00_00_41)]);
        v.shrink(10);
        v.assert(0, 0, &[]);

        v.grow(4);
        v.assert(128, 128, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        v.set(0);
        v.set(17);
        v.set(31);
        v.assert(128, 125, &[(0, 0x80_02_00_01), (1, 0), (2, 0), (3, 0)]);
        v.shrink(3);
        v.assert(32, 29, &[(0, 0x80_02_00_01)]);
        v.grow(3);
        v.assert(128, 125, &[(0, 0x80_02_00_01), (1, 0), (2, 0), (3, 0)]);
        v.set(32);
        v.assert(128, 124, &[(0, 0x80_02_00_01), (1, 0b1), (2, 0), (3, 0)]);
        v.shrink(2);
        v.assert(64, 60, &[(0, 0x80_02_00_01), (1, 0b1)]);
        v.shrink(1);
        v.assert(32, 29, &[(0, 0x80_02_00_01)]);
        v.grow(3);
        v.unset(31);
        v.set(33);
        v.set(34);
        v.set(96);
        v.set(105);
        v.assert(
            128,
            122,
            &[(0, 0x00_02_00_01), (1, 0b110), (2, 0), (3, 0b10_0000_0001)],
        );
        v.shrink(1);
        v.assert(96, 92, &[(0, 0x00_02_00_01), (1, 0b110), (2, 0)]);
        v.grow(1);
        v.set(97);
        v.set(100);
        v.set(127);
        v.assert(
            128,
            121,
            &[(0, 0x00_02_00_01), (1, 0b110), (2, 0), (3, 0x80_00_00_12)],
        );
        v.shrink(2);
        v.assert(64, 60, &[(0, 0x00_02_00_01), (1, 0b110)]);
        v.grow(2);
        v.set(96);
        v.set(99);
        v.assert(
            128,
            122,
            &[(0, 0x00_02_00_01), (1, 0b110), (2, 0), (3, 0b1001)],
        );
        v.shrink(3);
        v.assert(32, 30, &[(0, 0x00_02_00_01)]);
        v.grow(2);
        v.set(64);
        v.set(65);
        v.set(66);
        v.set(67);
        v.set(70);
        v.assert(96, 89, &[(0, 0x00_02_00_01), (1, 0), (2, 0b1001111)]);
        v.shrink(0);
        v.assert(96, 89, &[(0, 0x00_02_00_01), (1, 0), (2, 0b1001111)]);
        v.shrink(3);
        v.assert(0, 0, &[]);

        v.grow(3);
        v.set(32);
        v.set(35);
        v.assert(96, 94, &[(0, 0), (1, 0b1001), (2, 0)]);
        v.shrink(1);
        v.assert(64, 62, &[(0, 0), (1, 0b1001)]);
        v.grow(1);
        v.set(65);
        v.set(67);
        v.assert(96, 92, &[(0, 0), (1, 0b1001), (2, 0b1010)]);
        v.shrink(2);
        v.assert(32, 32, &[(0, 0)]);
        v.grow(5);
        v.set(190);
        v.set(160);
        v.assert(
            192,
            190,
            &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0x40_00_00_01)],
        );
        v.shrink(5);
        v.assert(32, 32, &[(0, 0)]);
        v.shrink(123);
        v.assert(0, 0, &[]);

        v.grow(1);
        for i in 0..32 {
            v.set(i);
        }
        v.assert(32, 0, &[(0, u32::MAX)]);
        v.shrink(1);
        v.assert(0, 0, &[]);

        v.grow(5);
        for i in 0..160 {
            v.set(i);
        }
        v.assert(
            160,
            0,
            &[
                (0, u32::MAX),
                (1, u32::MAX),
                (2, u32::MAX),
                (3, u32::MAX),
                (4, u32::MAX),
            ],
        );
        v.shrink(1);
        v.assert(
            128,
            0,
            &[(0, u32::MAX), (1, u32::MAX), (2, u32::MAX), (3, u32::MAX)],
        );
        v.shrink(2);
        v.assert(64, 0, &[(0, u32::MAX), (1, u32::MAX)]);
        v.shrink(2);
        v.assert(0, 0, &[]);

        v.grow(5);
        for i in 32..160 {
            v.set(i);
        }
        v.assert(
            160,
            32,
            &[
                (0, 0),
                (1, u32::MAX),
                (2, u32::MAX),
                (3, u32::MAX),
                (4, u32::MAX),
            ],
        );
        v.shrink(1);
        v.assert(
            128,
            32,
            &[(0, 0), (1, u32::MAX), (2, u32::MAX), (3, u32::MAX)],
        );
        v.shrink(1);
        v.assert(96, 32, &[(0, 0), (1, u32::MAX)]);
        v.shrink(2);
        v.assert(32, 32, &[(0, 0)]);
        v.shrink(1);
        v.assert(0, 0, &[]);

        v.grow(5);
        for i in 32..96 {
            v.set(i);
        }
        for i in 128..160 {
            v.set(i);
        }
        v.set(1);
        v.assert(
            160,
            63,
            &[
                (0, 0b10),
                (1, u32::MAX),
                (2, u32::MAX),
                (3, 0),
                (4, u32::MAX),
            ],
        );
        v.shrink(1);
        v.assert(128, 63, &[(0, 0b10), (1, u32::MAX), (2, u32::MAX), (3, 0)]);
        v.shrink(1);
        v.assert(96, 31, &[(0, 0b10), (1, u32::MAX), (2, u32::MAX)]);
        v.grow(1);
        v.assert(128, 63, &[(0, 0b10), (1, u32::MAX), (2, u32::MAX), (3, 0)]);
        v.shrink(2);
        v.assert(64, 31, &[(0, 0b10), (1, u32::MAX)]);
        v.shrink(1);
        v.assert(32, 31, &[(0, 0b10)]);
        v.grow(1);
        v.assert(64, 63, &[(0, 0b10), (1, 0)]);
        v.shrink(2);
        v.assert(0, 0, &[]);
    }

    #[test]
    fn set_unset() {
        let mut v = BitVec::<u16>::default();
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

    #[test]
    fn find() {
        let mut v = BitVec::<u16>::new();
        assert!(v.find().is_none());
        v.grow(1);
        assert_eq!(v.find(), Some(0));
        v.grow(2);
        assert_eq!(v.find(), Some(0));
        v.set(1);
        assert_eq!(v.find(), Some(0));
        v.set(2);
        assert_eq!(v.find(), Some(0));
        v.set(0);
        assert_eq!(v.find(), Some(3));
        v.set(5);
        assert_eq!(v.find(), Some(3));
        v.set(3);
        assert_eq!(v.find(), Some(4));
        v.unset(1);
        assert_eq!(v.find(), Some(1));
        v.unset(0);
        assert_eq!(v.find(), Some(0));
        v.set(0);
        v.set(1);
        v.set(4);
        assert_eq!(v.find(), Some(6));
        v.set(16);
        assert_eq!(v.find(), Some(6));
        v.set(32);
        assert_eq!(v.find(), Some(6));
        for i in 6..16 {
            v.set(i)
        }
        assert_eq!(v.find(), Some(17));
        v.set(18);
        assert_eq!(v.find(), Some(17));
        v.set(17);
        v.set(19);
        assert_eq!(v.find(), Some(20));
        for i in 20..30 {
            v.set(i);
        }
        assert_eq!(v.find(), Some(30));
        v.set(30);
        assert_eq!(v.find(), Some(31));
        v.set(31);
        assert_eq!(v.find(), Some(33));
        for i in 33..48 {
            v.set(i);
        }
        assert!(v.find().is_none());
        v.unset(16);
        assert_eq!(v.find(), Some(16));
        v.shrink(1);
        assert_eq!(v.find(), Some(16));
        v.shrink(1);
        assert!(v.find().is_none());
        v.unset(15);
        assert_eq!(v.find(), Some(15));
        v.unset(10);
        assert_eq!(v.find(), Some(10));
        v.unset(0);
        assert_eq!(v.find(), Some(0));
        v.grow(1);
        assert_eq!(v.find(), Some(0));
        v.shrink(2);
        assert!(v.find().is_none());
    }
}