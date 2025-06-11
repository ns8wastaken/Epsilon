use super::{
    PieceType,
    Square,
    Bitboard,
    Color,
    MoveType, Move,
    Board
};

use super::precomputed::{
    pawn,
    knight,
    bishop,
    rook,
    king
};

macro_rules! get_magic_moves {
    ($piece:ident, $square:ident, $occupied:ident) => {{
        let magic = &$piece::MAGICS[$square];
        magic.attacks[
            ((($occupied.all & magic.mask).wrapping_mul(magic.magic)) >> magic.shift) as usize
        ]
    }};
}

macro_rules! add_promotions {
    ($moves:ident, $from_square:ident, $to_square:ident) => {{
        $moves.push(Move::new($from_square, $to_square, MoveType::Promotion(PieceType::Queen)));
        $moves.push(Move::new($from_square, $to_square, MoveType::Promotion(PieceType::Rook)));
        $moves.push(Move::new($from_square, $to_square, MoveType::Promotion(PieceType::Bishop)));
        $moves.push(Move::new($from_square, $to_square, MoveType::Promotion(PieceType::Knight)));
    }}
}

pub struct MoveGen;

impl MoveGen {
    pub const fn attacks(board: &Board, piece_type: &PieceType, piece_color: &Color, square: Square) -> Bitboard {
        let square_idx = square as usize;
        let occupied = board.get_occupied();
        let enemy_occupied = occupied.enemy(piece_color);
        let en_passant_mask = match board.get_en_passant_square() {
            Some(sq) => 1u64 << sq,
            None     => 0,
        };

        match piece_type {
            PieceType::Pawn => pawn::ATTACKS[piece_color.index()][square_idx] & (enemy_occupied | en_passant_mask),

            PieceType::Knight => knight::ATTACKS_AND_MOVES[square_idx] & enemy_occupied,

            PieceType::Bishop => get_magic_moves!(bishop, square_idx, occupied) & enemy_occupied,

            PieceType::Rook => get_magic_moves!(rook, square_idx, occupied) & enemy_occupied,

            PieceType::Queen => (get_magic_moves!(rook, square_idx, occupied)
                                | get_magic_moves!(bishop, square_idx, occupied)) & enemy_occupied,

            PieceType::King => king::ATTACKS_AND_MOVES[square_idx] & enemy_occupied,
        }
    }

    pub const fn moves(board: &Board, piece_type: &PieceType, piece_color: &Color, square: Square) -> Bitboard {
        let square_idx = square as usize;
        let occupied = board.get_occupied();
        let unoccupied = !occupied.all;

        match piece_type {
            PieceType::Pawn => {
                let single = pawn::SINGLE_PUSH[piece_color.index()][square_idx] & unoccupied;
                let double = if single != 0 {
                    pawn::DOUBLE_PUSH[piece_color.index()][square_idx] & unoccupied
                } else {
                    0
                };
                single | double
            },

            PieceType::Knight => knight::ATTACKS_AND_MOVES[square_idx] & unoccupied,

            PieceType::Bishop => get_magic_moves!(bishop, square_idx, occupied) & unoccupied,

            PieceType::Rook => get_magic_moves!(rook, square_idx, occupied) & unoccupied,

            PieceType::Queen => (get_magic_moves!(rook, square_idx, occupied)
                                | get_magic_moves!(bishop, square_idx, occupied)) & unoccupied,

            PieceType::King => king::ATTACKS_AND_MOVES[square_idx] & unoccupied,
        }
    }

    pub fn get_pseudolegal_moves(board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();
        let en_passant_mask = match board.get_en_passant_square() {
            Some(sq) => 1u64 << sq,
            None     => 0,
        };

        for from_square in 0..64 {
            let Some(piece) = board.get_piece(from_square) else { continue; };
            if &piece.color != board.color_to_move() { continue; }

            if piece.piece_type == PieceType::King {
                if board.can_castle_kingside() {
                    moves.push(Move::new(from_square, from_square + 2, MoveType::CastleKingside));
                }
                if board.can_castle_queenside() {
                    moves.push(Move::new(from_square, from_square - 2, MoveType::CastleQueenside));
                }
            }

            // Quiet (non-capture) moves
            let mut quiet_moves = Self::moves(board, &piece.piece_type, &piece.color, from_square);
            while quiet_moves != 0 {
                let to_square = quiet_moves.trailing_zeros() as u8;
                quiet_moves &= quiet_moves - 1; // Pop lsb

                if (piece.piece_type == PieceType::Pawn) && (to_square < 8 || to_square >= 56) {
                    add_promotions!(moves, from_square, to_square);
                    continue;
                }

                moves.push(Move::new(from_square, to_square, MoveType::Quiet));
            }

            // Capture moves
            let mut capture_moves = Self::attacks(board, &piece.piece_type, &piece.color, from_square);
            while capture_moves != 0 {
                let to_square = capture_moves.trailing_zeros() as u8;
                capture_moves &= capture_moves - 1; // Pop lsb

                if piece.piece_type == PieceType::Pawn {
                    if to_square < 8 || to_square >= 56 {
                        add_promotions!(moves, from_square, to_square);
                        continue;
                    }

                    // En passant square is cleared after every move
                    // so no need to care about color / movement direction
                    if en_passant_mask & (1u64 << to_square) != 0 {
                        let from_to_dist = (to_square as i8 - from_square as i8).unsigned_abs();
                        if from_to_dist == 7 || from_to_dist == 9 {
                            moves.push(Move::new(from_square, to_square, MoveType::EnPassant));
                            continue;
                        }
                    }
                }
                moves.push(Move::new(from_square, to_square, MoveType::Capture));
            }
        }

        moves
    }
}
