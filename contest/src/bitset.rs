use std::{cmp::min, fmt::{Display, Debug}, ops::{BitAnd, BitOr, BitXor, BitAndAssign, BitOrAssign, BitXorAssign, Shl, ShlAssign, Index, IndexMut}};

use crate::{binary::range_one, num_integer::Integer};

const SHIFT: usize = 6;
const LOW: usize = 63;
const BITS_FOR_EACH: usize = 64;
const ALL_ONE: u64 = !0u64;
const ALL_ZERO: u64 = 0;
const MAX_OFFSET: u64 = 63;
const MIN_OFFSET: u64 = 0;

#[derive(Clone, Debug)]
pub struct BitSet {
    data: Vec<u64>,
    tail_available: u64,
    capacity: usize,
    m: usize,
}

impl BitSet {
    pub fn new(n: usize) -> Self {
        let capacity = n;
        let m = (capacity + 64 - 1) / 64;
        let data = vec![0; m];
        let tail_available = range_one(0u64, Self::offset(capacity - 1) as u64);
        Self {
            capacity,
            m,
            data,
            tail_available,
        }
    }

    pub fn range(&self, l: usize, r: usize) -> Self {
        let capacity = r - l + 1;
        let tail_available = range_one(0u64, Self::offset(capacity - 1) as u64);
        let req_length = Self::word(r) - Self::word(l) + 1;
        let data = self.data[Self::word(l)..Self::word(r) + 1].to_vec();
        let m = req_length;

        let mut res = Self {
            capacity,
            tail_available,
            data,
            m,
        };
        res.left_shift(Self::offset(l));
        res.m = (capacity + 64 - 1) / 64;
        res.data[m - 1] &= tail_available;
        for i in m..req_length {
            res.data[i] = 0;
        }

        res
    }
    pub fn left_shift(&mut self, n: usize) -> Self {
        let mut res = self.clone();
        res.left_shift_assign(n);
        res
    }
    pub fn right_shift(&mut self, n: usize) -> Self {
        let mut res = self.clone();
        res.right_shift_assign(n);
        res
    }
    pub fn left_shift_assign(&mut self, n: usize) {
        let word_move = Self::word(n);
        let offset_move = Self::offset(n) as u64;
        let rshift = MAX_OFFSET - (offset_move - 1);

        if offset_move != 0 {
            //slightly
            for i in 0..self.m {
                if i > 0 {
                    self.data[i - 1] |= self.data[i] << rshift;
                }
                self.data[i] >>= offset_move;
            }
        }
        if word_move > 0 {
            for i in 0..self.m {
                if i >= word_move {
                    self.data[i - word_move] = self.data[i];
                }
                self.data[i] = 0;
            }
        }
    }
    pub fn right_shift_assign(&mut self, n: usize) {
        let word_move = Self::word(n);
        let offset_move = Self::offset(n) as u64;
        let l_shift = MAX_OFFSET + 1 - offset_move;

        if offset_move != 0 {
            //slightly
            for i in (0..self.m).into_iter().rev() {
                if i + 1 < self.m {
                    self.data[i + 1] |= self.data[i] >> l_shift;
                }
                self.data[i] <<= offset_move;
            }
        }
        if word_move > 0 {
            for i in (0..self.m).into_iter().rev() {
                if i + word_move < self.m {
                    self.data[i + word_move] = self.data[i];
                }
                self.data[i] = 0;
            }
        }

        self.data[self.m - 1] &= self.tail_available;
    }

    fn word(i: usize) -> usize {
        return i >> SHIFT;
    }
    fn offset(i: usize) -> usize {
        i & LOW
    }

    pub fn get(&self, i: usize) -> bool {
        (self.data[Self::word(i)] & (1u64 << Self::offset(i) as u64)) != 0
    }
    pub fn set(&mut self, i: usize) {
        self.data[Self::word(i)] |= 1u64 << Self::offset(i);
    }
    pub fn clear(&mut self, i: usize) {
        self.data[Self::word(i)] &= !(1u64 << Self::offset(i));
    }
    pub fn flip(&mut self, i: usize) {
        self.data[Self::word(i)] ^= 1u64 << Self::offset(i);
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn range_clear(&mut self, l: usize, r: usize) {
        if r < l {
            return;
        }
        let l_word = l >> SHIFT as usize;
        let r_word = r >> SHIFT as usize;
        for i in l_word + 1..r_word {
            self.data[i] = ALL_ZERO;
        }
        //lword
        if l_word == r_word {
            self.data[l_word] &= !range_one(Self::offset(l) as u64, Self::offset(r) as u64);
        } else {
            self.data[l_word] &= !range_one(Self::offset(l) as u64, MAX_OFFSET);
            self.data[r_word] &= !range_one(0, Self::offset(r) as u64);
        }
    }

    pub fn range_flip(&mut self, l: usize, r: usize) {
        if r < l {
            return;
        }
        let l_word = l >> SHIFT as usize;
        let r_word = r >> SHIFT as usize;
        for i in l_word + 1..r_word {
            self.data[i] ^= ALL_ONE;
        }
        //lword
        if l_word == r_word {
            self.data[l_word] ^= range_one(Self::offset(l) as u64, Self::offset(r) as u64);
        } else {
            self.data[l_word] ^= range_one(Self::offset(l) as u64, MAX_OFFSET);
            self.data[r_word] ^= range_one(0, Self::offset(r) as u64);
        }
    }

    pub fn range_set(&mut self, l: usize, r: usize) {
        if r < l {
            return;
        }
        let l_word = l >> SHIFT as usize;
        let r_word = r >> SHIFT as usize;
        for i in l_word + 1..r_word {
            self.data[i] = ALL_ONE;
        }
        //lword
        if l_word == r_word {
            self.data[l_word] |= range_one(Self::offset(l) as u64, Self::offset(r) as u64);
        } else {
            self.data[l_word] |= range_one(Self::offset(l) as u64, MAX_OFFSET);
            self.data[r_word] |= range_one(0, Self::offset(r) as u64);
        }
    }

    pub fn size(&self) -> usize {
        self.range_size(0, self.capacity() - 1)
    }

    pub fn range_size(&self, l: usize, r: usize) -> usize {
        if r < l {
            return 0;
        }
        let mut ans = 0;
        let l_word = l >> SHIFT;
        let r_word = r >> SHIFT;
        for i in l_word + 1..r_word {
            ans += self.data[i].bit_count();
        }
        //lword
        if l_word == r_word {
            ans += (self.data[l_word] & range_one(Self::offset(l) as u64, Self::offset(r) as u64))
                .bit_count();
        } else {
            ans += (self.data[l_word] & range_one(Self::offset(l) as u64, MAX_OFFSET)).bit_count();
            ans += (self.data[r_word] & range_one(0, Self::offset(r) as u64)).bit_count();
        }
        return ans as usize;
    }

    pub fn or(&mut self, bs: &Self) {
        let n = min(self.m, bs.m);
        for i in 0..n {
            self.data[i] |= bs.data[i];
        }
    }

    pub fn and(&mut self, bs: &Self) {
        let n = min(self.m, bs.m);
        for i in 0..n {
            self.data[i] &= bs.data[i];
        }
    }

    pub fn xor(&mut self, bs: &Self) {
        let n = min(self.m, bs.m);
        for i in 0..n {
            self.data[i] ^= bs.data[i];
        }
    }

    fn next_bit(&self, start: usize, xor: u64) -> Option<usize> {
        let offset = Self::offset(start) as u64;
        let mut w = Self::word(start);
        if offset != 0 {
            let mask = range_one(offset, MAX_OFFSET);
            if ((self.data[w] ^ xor) & mask) != 0 {
                return Some(
                    ((self.data[w] ^ xor) & mask).count_trailing_zero() as usize
                        + w * BITS_FOR_EACH,
                );
            }
            w += 1;
        }

        while w < self.m && (self.data[w] ^ xor) == ALL_ZERO {
            w += 1;
        }
        if w >= self.m {
            return None;
        }
        return Some((self.data[w] ^ xor).count_trailing_zero() as usize + w * BITS_FOR_EACH);
    }

    pub fn next_set_bit(&self, start: usize) -> Option<usize> {
        self.next_bit(start, ALL_ZERO)
    }
    pub fn next_clear_bit(&self, start: usize) -> Option<usize> {
        self.next_bit(start, ALL_ONE)
    }
    pub fn previous_bit(&self, start: usize, xor: u64) -> Option<usize> {
        let offset = Self::offset(start) as u64;
        let mut w = Self::word(start);
        if offset != MAX_OFFSET {
            let mask = range_one(0, offset);
            if ((self.data[w] ^ xor) & mask) != 0 {
                return Some(
                    MAX_OFFSET as usize
                        - ((self.data[w] ^ xor) & mask).count_leading_zero() as usize
                        + w * BITS_FOR_EACH,
                );
            }
            w -= 1;
        }

        while w != usize::MAX && (self.data[w] ^ xor) == ALL_ZERO {
            w -= 1;
        }
        if w == usize::MAX {
            return None;
        }
        Some(
            MAX_OFFSET as usize - (self.data[w] ^ xor).count_leading_zero() as usize
                + w * BITS_FOR_EACH,
        )
    }
    pub fn previous_set_bit(&self, start: usize) -> Option<usize> {
        self.previous_bit(start, ALL_ZERO)
    }
    pub fn previous_clear_bit(&self, start: usize) -> Option<usize> {
        self.previous_bit(start, ALL_ONE)
    }

    pub fn iter_one<'a>(&'a self) -> BitSetIterator<'a> {
        self.iter(ALL_ZERO)
    }

    pub fn iter_zero<'a>(&'a self) -> BitSetIterator<'a> {
        self.iter(ALL_ONE)
    }

    fn iter<'a>(&'a self, xor: u64) -> BitSetIterator<'a> {
        BitSetIterator {
            bitset: self,
            xor,
            lr: Some(0),
            rl: Some(self.capacity() - 1),
        }
    }
}

pub struct BitSetIterator<'a> {
    bitset: &'a BitSet,
    xor: u64,
    lr: Option<usize>,
    rl: Option<usize>,
}
impl<'a> Iterator for BitSetIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lr {
            None => None,
            Some(x) => match self.bitset.next_bit(x, self.xor) {
                None => None,
                Some(x) => {
                    self.lr = Some(x + 1);
                    Some(x)
                }
            },
        }
    }
}
impl<'a> DoubleEndedIterator for BitSetIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match self.rl {
            None => None,
            Some(x) => match self.bitset.previous_bit(x, self.xor) {
                None => None,
                Some(x) => {
                    self.rl = if x == 0 { None } else { Some(x - 1) };
                    Some(x)
                }
            },
        }
    }
}
impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data: Vec<usize> = self.iter_one().collect();
        data.fmt(f)
    }
}
impl BitAnd for BitSet {
    type Output = BitSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.and(&rhs);
        res
    }
}
impl BitAndAssign for BitSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.and(&rhs)
    }
}

impl BitOr for BitSet {
    type Output = BitSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.or(&rhs);
        res
    }
}

impl BitOrAssign for BitSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.or(&rhs)
    }
}

impl BitXor for BitSet {
    type Output = BitSet;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let mut res = self.clone();
        res.xor(&rhs);
        res
    }
}
impl BitXorAssign for BitSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.xor(&rhs)
    }
}
impl Index<usize> for BitSet {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        if self.get(index) {
            &true
        } else {
            &false
        }
    }
}