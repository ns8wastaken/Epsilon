use crate::board::{
    Bitboard,
    CastlingRights,
    Color,
    Mailbox,
    Occupied,
    Square
};

pub struct HistoryState {
    pub bitboards: [Bitboard; 12],
    pub mailbox: Mailbox,
    pub occupied: Occupied,
    pub color_to_move: Color,
    pub en_passant_square: Option<Square>,
    pub castling_rights: CastlingRights,
}
