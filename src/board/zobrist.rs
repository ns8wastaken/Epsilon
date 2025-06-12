use rand::Rng;
use super::PIECETYPE_COUNT;

pub struct Zobrist {
    pub pieces: [u64; 64 * PIECETYPE_COUNT * 2],
    pub side_to_move: u64,
    pub castling_rights: [u64; 4],   // [White K, White Q, Black K, Black Q]
    pub en_passant_file: [u64; 8],   // en passant file a-h
}

impl Zobrist {
    pub fn new() -> Self {
        let mut rng = rand::rng();

        let mut pieces = [0; 64 * PIECETYPE_COUNT * 2];
        for color in 0..2 {
            for piece_type in 0..PIECETYPE_COUNT {
                for square in 0..64 {
                    pieces[Self::index(color, piece_type, square)] = rng.random();
                }
            }
        }

        let castling_rights = std::array::from_fn(|_| rng.random());
        let en_passant_file = std::array::from_fn(|_| rng.random());

        Self {
            pieces,
            side_to_move: rng.random(),
            castling_rights,
            en_passant_file,
        }
    }

    #[inline(always)]
    pub const fn index(color: usize, piece_type: usize, square: u8) -> usize {
        debug_assert!(color < 2);
        debug_assert!(piece_type < PIECETYPE_COUNT);
        debug_assert!(square < 64);
        (color * PIECETYPE_COUNT + piece_type) * 64 + square as usize
    }
}
