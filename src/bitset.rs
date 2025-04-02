use std::{
    fmt::Display,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bitset(u64);

impl Bitset {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get_bit(&self, index: u8) -> u64 {
        self.0 & (1 << index)
    }

    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub fn is_bit_set(&self, index: u8) -> bool {
        self.get_bit(index) != 0
    }

    pub fn set_bit_indices(&self) -> Vec<u8> {
        let mut indices = Vec::new();
        for index in 0..64 {
            if self.is_bit_set(index) {
                indices.push(index);
            }
        }
        indices
    }
}

impl BitAnd for Bitset {
    type Output = Bitset;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitset::new(self.0 & rhs.0)
    }
}

impl BitOr for Bitset {
    type Output = Bitset;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bitset::new(self.0 | rhs.0)
    }
}

impl BitAndAssign for Bitset {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = Bitset::new(self.0 & rhs.0)
    }
}

impl BitOrAssign for Bitset {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = Bitset::new(self.0 | rhs.0)
    }
}

impl Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}
