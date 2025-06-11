#[derive(Clone, Copy, Debug)]
pub struct CastlingRights {
    pub white_queen_side: bool,
    pub white_king_side: bool,
    pub black_queen_side: bool,
    pub black_king_side: bool,
}

impl CastlingRights {
    pub const fn default() -> Self {
        Self {
            white_queen_side: true,
            white_king_side: true,
            black_queen_side: true,
            black_king_side: true
        }
    }

    pub const fn default_but_false() -> Self {
        Self {
            white_queen_side: false,
            white_king_side: false,
            black_queen_side: false,
            black_king_side: false
        }
    }
}
