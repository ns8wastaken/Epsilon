use super::{Bitboard, utils};

pub const ATTACKS_AND_MOVES: [Bitboard; 64] = {
    let mut moves: [Bitboard; 64] = [0u64; 64];

    let mut square = 0usize;
    while square < 64 {
        let pos = 1u64 << square;

        moves[square] =
            (pos & utils::BIT_MASK_B) << 9
            | pos << 8
            | (pos & utils::BIT_MASK_A) << 7
            | (pos & utils::BIT_MASK_B) << 1
            | (pos & utils::BIT_MASK_A) >> 1
            | (pos & utils::BIT_MASK_B) >> 7
            | pos >> 8
            | (pos & utils::BIT_MASK_A) >> 9;

        square += 1;
    }

    moves
};
