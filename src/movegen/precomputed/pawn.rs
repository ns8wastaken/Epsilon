use super::{Bitboard, Color};

pub const ATTACKS: [[Bitboard; 64]; 2] = {
    let mut attacks = [[0u64; 64]; 2];

    let mut square = 0usize;
    while square < 64 {
        let rank = square / 8;
        let file = square % 8;

        let mut white_bb = 0u64;
        if rank < 7 {
            if file > 0 { white_bb |= 1u64 << (square + 7); }
            if file < 7 { white_bb |= 1u64 << (square + 9); }
        }

        let mut black_bb = 0u64;
        if rank > 0 {
            if file > 0 { black_bb |= 1u64 << (square - 9); }
            if file < 7 { black_bb |= 1u64 << (square - 7); }
        }

        attacks[Color::White.index()][square] = white_bb;
        attacks[Color::Black.index()][square] = black_bb;

        square += 1;
    }

    attacks
};

pub const SINGLE_PUSH: [[Bitboard; 64]; 2] = {
    let mut pushes = [[0u64; 64]; 2];

    let mut square = 0usize;
    while square < 64 {
        let rank = square / 8;

        let mut white_bb = 0u64;
        if rank < 7 {
            white_bb |= 1u64 << (square + 8);
        }

        let mut black_bb = 0u64;
        if rank > 0 {
            black_bb |= 1u64 << (square - 8);
        }

        pushes[Color::White.index()][square] = white_bb;
        pushes[Color::Black.index()][square] = black_bb;

        square += 1;
    }

    pushes
};

pub const DOUBLE_PUSH: [[Bitboard; 64]; 2] = {
    let mut pushes = [[0u64; 64]; 2];

    let mut square = 0usize;
    while square < 64 {
        let rank = square / 8;

        let mut white_bb = 0u64;
        if rank == 1 {
            white_bb |= 1u64 << (square + 16);
        }

        let mut black_bb = 0u64;
        if rank == 6 {
            black_bb |= 1u64 << (square - 16);
        }

        pushes[Color::White.index()][square] = white_bb;
        pushes[Color::Black.index()][square] = black_bb;

        square += 1;
    }

    pushes
};
