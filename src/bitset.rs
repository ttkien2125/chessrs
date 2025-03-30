use std::fmt::Display;

pub struct Bitset(u64);

impl Bitset {
    pub const fn new(value: u64) -> Self {
        Bitset(value)
    }

    pub fn set_bit(&mut self, square: u8) {
        self.0 |= 1 << square;
    }

    pub fn clear_bit(&mut self, square: u8) {
        self.0 &= !(1 << square);
    }

    pub fn is_bit_set(&self, square: u8) -> bool {
        (self.0 & (1 << square)) != 0
    }
}

impl Display for Bitset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#018x}", self.0)
    }
}
