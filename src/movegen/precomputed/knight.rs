use super::{Bitboard, utils};

pub const ATTACKS_AND_MOVES: [Bitboard; 64] = {
    let mut moves: [Bitboard; 64] = [0u64; 64];

    let mut square = 0usize;
    while square < 64 {
        let pos = 1u64 << square;

        moves[square] =
            (pos & utils::BIT_MASK_B)    << 17
            | (pos & utils::BIT_MASK_A)  << 15
            | (pos & utils::BIT_MASK_B2) << 10
            | (pos & utils::BIT_MASK_A2) << 6
            | (pos & utils::BIT_MASK_B2) >> 6
            | (pos & utils::BIT_MASK_A2) >> 10
            | (pos & utils::BIT_MASK_B)  >> 15
            | (pos & utils::BIT_MASK_A)  >> 17;

        square += 1;
    }

    moves
};
