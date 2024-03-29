//! Vector of bits.

use std::fmt;
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

    /// Creates a bit vector with an initial number of `T`s.
    /// The total number of bits in the vector will be equal
    /// to `n * T::BITS`.
    /// All bits are initially unset.
    pub fn with_count_words(n: usize) -> Self {
        if n == 0 {
            Self::new()
        } else {
            Self {
                vec: vec![T::ZERO; n],
                rem: n * T::BITS,
            }
        }
    }

    /// Increment the vector by `inc` units.
    /// The unit of the increment is `T`. The number of bits
    /// pushed will be equal to `inc * T::BITS`.
    /// It returns the index of the first bit in the pushed
    /// range, or `None` if `inc` was `0`.
    pub fn grow(&mut self, inc: usize) -> Option<usize> {
        let bit_idx = self.len();
        self.rem += inc * T::BITS;
        match inc {
            0 => None,
            1 => {
                self.vec.push(T::ZERO);
                Some(bit_idx)
            }
            _ => {
                self.vec.resize(self.vec.len() + inc, T::ZERO);
                Some(bit_idx)
            }
        }
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
                    if u == T::ZERO {
                        self.rem -= T::BITS;
                    } else if u != !T::ZERO {
                        u = !u;
                        while u != T::ZERO {
                            if u & T::ONE == T::ONE {
                                self.rem -= 1;
                            }
                            u >>= 1;
                        }
                    }
                }
            }
        }
    }

    /// Trims the vector to the minimum possible length.
    /// This method pops `T`s from the vector until an
    /// element with a bit set is found.
    /// It returns the number of `T`s popped.
    pub fn trim(&mut self) -> usize {
        let vec_len = self.vec.len();
        let bit_len = self.len();
        // This first condition makes the `else` unreachable.
        if bit_len == self.rem {
            self.vec.clear();
            self.rem = 0;
            vec_len
        } else if self.rem < T::BITS {
            0
        } else if let Some(i) = self.vec.iter().rposition(|&x| x != T::ZERO) {
            let n = vec_len - (i + 1);
            if n != 0 {
                self.vec.truncate(i + 1);
                self.rem -= n * T::BITS;
            }
            n
        } else {
            unreachable!();
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

    /// Searches for a contiguous range of unset bits.
    /// It returns the index of the first unset bit in the
    /// range, or `None` if there is no contiguous, unset
    /// range of size `n`.
    /// The bits are not set by this method.
    pub fn find_contiguous(&self, n: usize) -> Option<usize> {
        match n {
            0..=1 => self.find(),
            x if x > self.rem => None,
            _ => {
                // Number of contiguous bits found.
                let mut cnt = 0;
                // Index of the element where the range begins.
                let mut idx = 0;
                // Bit offset within `idx`.
                let mut bit = 0;
                // Index of the element being checked.
                let mut i = 0;
                loop {
                    // Skip fully set elements.
                    if self.vec[i] == !T::ZERO {
                        cnt = 0;
                        bit = 0;
                        i += 1;
                        for _ in i..self.vec.len() {
                            if self.vec[i] != !T::ZERO {
                                break;
                            }
                            i += 1;
                        }
                        idx = i;
                    }

                    // Give up if there is not enough bits left.
                    if cnt + T::BITS * (self.vec.len() - i) < n {
                        break None;
                    }

                    // Iterate over whole elements as much as possible.
                    if self.vec[i] == T::ZERO {
                        cnt += T::BITS;
                        i += 1;
                        if cnt < n {
                            for j in 0..(n - cnt) / T::BITS {
                                if self.vec[i + j] != T::ZERO {
                                    cnt += j * T::BITS;
                                    i += j;
                                    break;
                                }
                            }
                        }
                        if cnt >= n {
                            break Some(idx * T::BITS + bit);
                        }
                    }

                    // Iterate over the bits of the ith element.
                    // There are three possibilities:
                    //
                    // 1. It completes a range (i.e., bits 0..n-cnt are
                    //    unset) or
                    // 2. There is a range of n unset bits contained
                    //    within this element or
                    // 3. It has a (possibly empty) subrange x..T::BITS
                    //    of unset bits that may yet form a full range
                    //    with subsequent element(s).
                    for j in 0..T::BITS {
                        if self.vec[i] & (T::ONE << j) == T::ZERO {
                            cnt += 1;
                            if cnt >= n {
                                return Some(idx * T::BITS + bit);
                            }
                        } else {
                            cnt = 0;
                            if j < T::BITS - 1 {
                                idx = i;
                                bit = j + 1;
                            } else {
                                idx = i + 1;
                                bit = 0;
                            }
                        }
                    }
                    i += 1;
                    if i == self.vec.len() {
                        break None;
                    }
                }
            }
        }
    }

    /// Returns the vector's length in number of bits.
    pub fn len(&self) -> usize {
        self.vec.len() * T::BITS
    }

    /// Returns the number of unset bits.
    pub fn rem(&self) -> usize {
        self.rem
    }
}

impl<T: Unsigned> Default for BitVec<T> {
    /// Calls [`BitVec::new`].
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Unsigned + fmt::Binary> fmt::Display for BitVec<T> {
    /// Formats the bit vector to display the bits themselves.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = T::BITS;
        writeln!(f, "{bits}-bit BitVec")?;

        let wdt10 = {
            let mut n = self.len() as isize - T::BITS as isize;
            let mut w = 1;
            while n >= 10 {
                n /= 10;
                w += 1;
            }
            w
        };

        for (i, x) in self.vec.iter().enumerate() {
            let beg = i * T::BITS;
            let end = beg + T::BITS - 1;
            writeln!(f, "    {x:0bits$b} {beg:wdt10$} ..= {end}")?;
        }

        writeln!(f, "Number of bits: {}", self.len())?;
        writeln!(f, "Remaining bits: {}", self.rem())
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
        assert_eq!(v.grow(1), Some(0));
        v.assert(64, 64, &[(0, 0)]);
        assert_eq!(v.grow(1), Some(64));
        v.assert(128, 128, &[(0, 0), (1, 0)]);
        assert_eq!(v.grow(3), Some(128));
        v.assert(320, 320, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);

        let mut v: BitVec<u8> = BitVec::new();
        v.assert(0, 0, &[]);
        assert_eq!(v.grow(4), Some(0));
        v.assert(32, 32, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(v.grow(1), Some(32));
        v.assert(40, 40, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
        assert_eq!(v.grow(2), Some(40));
        v.assert(
            56,
            56,
            &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)],
        );

        let mut v = <BitVec<usize>>::new();
        v.assert(0, 0, &[]);
        assert!(v.grow(0).is_none());
        v.assert(0, 0, &[]);
        assert_eq!(v.grow(1), Some(0));
        v.assert(usize::BITS as _, usize::BITS as _, &[(0, 0)]);
        assert_eq!(v.grow(2), Some(usize::BITS as _));
        v.assert(
            usize::BITS as usize * 3,
            usize::BITS as usize * 3,
            &[(0, 0), (1, 0)],
        );

        let mut v = BitVec::<u32>::with_count_words(1);
        v.assert(32, 32, &[(0, 0)]);
        assert_eq!(v.grow(1), Some(32));
        v.assert(64, 64, &[(0, 0), (1, 0)]);
        assert_eq!(v.grow(3), Some(64));
        v.assert(160, 160, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)]);
        assert_eq!(v.grow(1), Some(160));
        v.assert(192, 192, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]);

        let mut v = BitVec::<u16>::with_count_words(0);
        v.assert(0, 0, &[]);
        assert_eq!(v.grow(3), Some(0));
        v.assert(48, 48, &[(0, 0), (1, 0), (2, 0)]);

        let mut v = BitVec::<u128>::with_count_words(4);
        v.assert(512, 512, &[(0, 0), (1, 0), (2, 0), (3, 0)]);
        assert_eq!(v.grow(2), Some(512));
        v.assert(768, 768, &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0)]);
        assert_eq!(v.grow(1), Some(768));
        v.assert(
            896,
            896,
            &[(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)],
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
    fn trim() {
        let mut v = BitVec::<u8>::new();
        assert_eq!(v.trim(), 0);
        v.assert(0, 0, &[]);
        v.grow(1);
        assert_eq!(v.trim(), 1);
        v.assert(0, 0, &[]);
        v.grow(1);
        v.set(0);
        assert_eq!(v.trim(), 0);
        v.assert(8, 7, &[(0, 0b1)]);
        v.grow(1);
        assert_eq!(v.trim(), 1);
        v.assert(8, 7, &[(0, 0b1)]);
        v.grow(1);
        (1..8).for_each(|i| v.set(i));
        assert_eq!(v.trim(), 1);
        v.assert(8, 0, &[(0, 0xff)]);
        v.grow(1);
        v.set(10);
        assert_eq!(v.trim(), 0);
        v.assert(16, 7, &[(0, 0xff), (1, 0b100)]);
        v.grow(2);
        v.set(15);
        assert_eq!(v.trim(), 2);
        v.assert(16, 6, &[(0, 0xff), (1, 0b10000100)]);
        v.grow(2);
        v.set(29);
        assert_eq!(v.trim(), 0);
        v.assert(32, 21, &[(0, 0xff), (1, 0x84), (2, 0), (3, 0x20)]);
        v.grow(3);
        v.set(24);
        assert_eq!(v.trim(), 3);
        v.assert(32, 20, &[(0, 0xff), (1, 0x84), (2, 0), (3, 0x21)]);
        v.grow(3);
        v.unset(29);
        assert_eq!(v.trim(), 3);
        v.assert(32, 21, &[(0, 0xff), (1, 0x84), (2, 0), (3, 0b1)]);
        v.unset(24);
        v.grow(3);
        assert_eq!(v.trim(), 5);
        v.assert(16, 6, &[(0, 0xff), (1, 0x84)]);
        (0..15).for_each(|i| v.unset(i));
        assert_eq!(v.trim(), 0);
        v.assert(16, 15, &[(0, 0), (1, 0x80)]);
        v.unset(15);
        assert_eq!(v.trim(), 2);
        v.assert(0, 0, &[]);

        let mut v = BitVec::<u16>::default();
        v.grow(10);
        assert_eq!(v.trim(), 10);
        v.assert(0, 0, &[]);
        v.grow(10);
        v.set(75);
        assert_eq!(v.trim(), 5);
        v.assert(80, 79, &[(4, 0x0800)]);
        assert_eq!(v.trim(), 0);
        v.assert(80, 79, &[(4, 0x0800)]);
        v.unset(75);
        assert_eq!(v.trim(), 5);
        v.assert(0, 0, &[]);

        let mut v = BitVec::<u32>::default();
        v.grow(9);
        v.set(287);
        v.set(272);
        v.set(271);
        v.set(256);
        v.set(193);
        assert_eq!(v.trim(), 0);
        v.assert(288, 283, &[(8, 0x80018001), (6, 0b10)]);
        v.unset(287);
        assert_eq!(v.trim(), 0);
        v.assert(288, 284, &[(8, 0x00018001), (6, 0b10)]);
        v.unset(256);
        assert_eq!(v.trim(), 0);
        v.assert(288, 285, &[(8, 0x00018000), (6, 0b10)]);
        v.unset(271);
        assert_eq!(v.trim(), 0);
        v.assert(288, 286, &[(8, 0x00010000), (6, 0b10)]);
        v.unset(272);
        assert_eq!(v.trim(), 2);
        v.assert(224, 223, &[(6, 0b10)]);
        v.unset(193);
        assert_eq!(v.trim(), 7);
        v.assert(0, 0, &[]);

        let mut v = BitVec::<u64>::new();
        v.grow(50);
        v.set(31);
        v.set(640);
        v.set(2047);
        assert_eq!(v.trim(), (3200 - 2048) / 64);
        v.assert(2048, 2045, &[(0, 1 << 31), (10, 1), (31, 1 << 63)]);
        v.unset(640);
        assert_eq!(v.trim(), 0);
        v.assert(2048, 2046, &[(0, 1 << 31), (10, 0), (31, 1 << 63)]);
        v.unset(2047);
        assert_eq!(v.trim(), (2048 - 64) / 64);
        v.assert(64, 63, &[(0, 1 << 31)]);
        v.unset(31);
        assert_eq!(v.trim(), 1);
        v.assert(0, 0, &[]);
        assert_eq!(v.trim(), 0);
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

    #[test]
    fn find_contiguous() {
        let mut v = BitVec::<u16>::new();
        assert!(v.find_contiguous(1).is_none());
        assert!(v.find_contiguous(16).is_none());
        assert!(v.find_contiguous(17).is_none());
        v.grow(1);
        for i in 1..16 {
            assert_eq!(v.find_contiguous(i), Some(0));
        }
        assert!(v.find_contiguous(17).is_none());
        v.grow(1);
        for i in 1..32 {
            assert_eq!(v.find_contiguous(i), Some(0));
        }
        assert!(v.find_contiguous(33).is_none());

        v.set(1);
        assert_eq!(v.find_contiguous(1), Some(0));
        assert_eq!(v.find_contiguous(2), Some(2));
        assert_eq!(v.find_contiguous(3), Some(2));
        assert_eq!(v.find_contiguous(16), Some(2));
        v.set(4);
        assert_eq!(v.find_contiguous(2), Some(2));
        assert_eq!(v.find_contiguous(3), Some(5));
        assert_eq!(v.find_contiguous(16), Some(5));
        v.set(9);
        assert_eq!(v.find_contiguous(2), Some(2));
        assert_eq!(v.find_contiguous(3), Some(5));
        assert_eq!(v.find_contiguous(4), Some(5));
        assert_eq!(v.find_contiguous(5), Some(10));
        assert_eq!(v.find_contiguous(22), Some(10));
        assert!(v.find_contiguous(23).is_none());
        v.set(16);
        assert_eq!(v.find_contiguous(1), Some(0));
        assert_eq!(v.find_contiguous(2), Some(2));
        assert_eq!(v.find_contiguous(6), Some(10));
        assert_eq!(v.find_contiguous(7), Some(17));
        assert_eq!(v.find_contiguous(15), Some(17));
        assert!(v.find_contiguous(16).is_none());
        v.grow(1);
        assert_eq!(v.find_contiguous(16), Some(17));
        v.set(33);
        assert_eq!(v.find_contiguous(16), Some(17));
        v.set(32);
        assert!(v.find_contiguous(16).is_none());
        v.unset(16);
        assert_eq!(v.find_contiguous(16), Some(10));
        v.unset(32);
        assert_eq!(v.find_contiguous(16), Some(10));
        v.set(17);
        assert!(v.find_contiguous(16).is_none());
        v.unset(33);
        assert_eq!(v.find_contiguous(16), Some(18));
        v.set(20);
        v.set(40);
        assert!(v.find_contiguous(20).is_none());
        v.unset(40);
        assert_eq!(v.find_contiguous(20), Some(21));
        v.unset(20);
        assert_eq!(v.find_contiguous(20), Some(18));
        assert!(v.find_contiguous(31).is_none());
        v.unset(17);
        v.set(13);
        assert_eq!(v.find_contiguous(31), Some(14));
        v.set(14);
        assert_eq!(v.find_contiguous(31), Some(15));
        v.set(46);
        assert_eq!(v.find_contiguous(31), Some(15));
        assert!(v.find_contiguous(32).is_none());
        v.unset(46);
        v.set(47);
        assert_eq!(v.find_contiguous(31), Some(15));
        assert!(v.find_contiguous(33).is_none());
        v.unset(14);
        assert_eq!(v.find_contiguous(33), Some(14));
        v.set(0);
        v.unset(1);
        assert_eq!(v.find_contiguous(33), Some(14));
        v.unset(13);
        assert_eq!(v.find_contiguous(34), Some(10));
        assert!(v.find_contiguous(38).is_none());
        v.unset(47);
        assert_eq!(v.find_contiguous(38), Some(10));

        v.shrink(3);
        v.grow(3);
        assert_eq!(v.find_contiguous(16), Some(0));
        assert_eq!(v.find_contiguous(32), Some(0));
        assert_eq!(v.find_contiguous(48), Some(0));
        assert!(v.find_contiguous(64).is_none());
        for i in (0..16).chain(32..48) {
            v.set(i);
        }
        assert!(v.find_contiguous(17).is_none());
        assert_eq!(v.find_contiguous(1), Some(16));
        assert_eq!(v.find_contiguous(15), Some(16));
        assert_eq!(v.find_contiguous(16), Some(16));
        v.set(16);
        assert_eq!(v.find_contiguous(1), Some(17));
        assert_eq!(v.find_contiguous(2), Some(17));
        assert_eq!(v.find_contiguous(15), Some(17));
        assert!(v.find_contiguous(16).is_none());

        v.shrink(3);
        v.grow(3);
        for i in (0..15).chain(18..30).chain(33..48) {
            v.set(i);
        }
        assert!(v.find_contiguous(4).is_none());
        assert_eq!(v.find_contiguous(1), Some(15));
        assert_eq!(v.find_contiguous(2), Some(15));
        assert_eq!(v.find_contiguous(3), Some(15));
        v.set(17);
        assert_eq!(v.find_contiguous(1), Some(15));
        assert_eq!(v.find_contiguous(2), Some(15));
        assert_eq!(v.find_contiguous(3), Some(30));
        v.set(16);
        assert_eq!(v.find_contiguous(1), Some(15));
        assert_eq!(v.find_contiguous(2), Some(30));
        assert_eq!(v.find_contiguous(3), Some(30));
        v.set(15);
        v.set(18);
        assert_eq!(v.find_contiguous(1), Some(30));
        assert_eq!(v.find_contiguous(2), Some(30));
        assert_eq!(v.find_contiguous(3), Some(30));
        v.set(31);
        assert_eq!(v.find_contiguous(1), Some(30));
        assert!(v.find_contiguous(2).is_none());
        assert!(v.find_contiguous(3).is_none());

        v.shrink(9999);
        v.grow(10);
        for i in 0..160 {
            assert_eq!(v.find_contiguous(160 - i), Some(i));
            v.set(i);
        }
        assert!(v.find_contiguous(1).is_none());
        for i in 0..160 {
            v.unset(159 - i);
            assert_eq!(v.find_contiguous(i + 1), Some(159 - i));
        }
        assert_eq!(v.find_contiguous(160), Some(0));
    }

    #[test]
    #[ignore]
    fn fmt() {
        let mut v8 = BitVec::<u8>::new();
        v8.grow(10);
        v8.set(0);
        v8.set(v8.len() / 2);
        v8.set(v8.len() - 1);
        println!("{v8}");

        let mut v16 = BitVec::<u16>::new();
        v16.grow(5);
        v16.set(0);
        v16.set(v16.len() / 2);
        v16.set(v16.len() - 1);
        println!("{v16}");

        let mut v32 = BitVec::<u32>::new();
        v32.grow(21);
        v32.set(0);
        v32.set(v32.len() / 2);
        v32.set(v32.len() - 1);
        println!("{v32}");

        let mut v64 = BitVec::<u64>::new();
        v64.grow(16);
        v64.set(0);
        v64.set(v64.len() / 2);
        v64.set(v64.len() - 1);
        println!("{v64}");

        let mut v128 = BitVec::<u128>::new();
        v128.grow(9);
        v128.set(0);
        v128.set(v128.len() / 2);
        v128.set(v128.len() - 1);
        println!("{v128}");

        let mut vsz = BitVec::<usize>::new();
        vsz.grow(1);
        vsz.set(0);
        vsz.set(vsz.len() / 2);
        vsz.set(vsz.len() - 1);
        println!("{vsz}");
    }
}
