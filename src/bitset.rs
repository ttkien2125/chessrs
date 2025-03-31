use std::fmt::Display;

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
}

impl Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}
