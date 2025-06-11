use super::bitboard::Bitboard;
use super::color::Color;

#[derive(Clone, Copy)]
pub struct Occupied {
    pub white: Bitboard,
    pub black: Bitboard,
    pub all: Bitboard,
}

impl Occupied {
    pub const fn new() -> Self {
        Self { white: 0, black: 0, all: 0 }
    }

    #[inline(always)]
    pub const fn enemy(&self, color: &Color) -> Bitboard {
        match color {
            Color::White => self.black,
            Color::Black => self.white,
        }
    }
}
