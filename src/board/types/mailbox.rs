use super::pieces::{Piece, PieceType};
use super::color::Color;
use super::square::Square;

#[derive(Clone, Copy)]
pub struct Mailbox([Option<Piece>; 64]);

impl Mailbox {
    pub const fn new() -> Self {
        Self([
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
        ])
    }

    pub const fn startpos() -> Self {
        Self([
            Some(Piece { piece_type: PieceType::Rook,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
            Some(Piece { piece_type: PieceType::Queen,  color: Color::White }),
            Some(Piece { piece_type: PieceType::King,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
            Some(Piece { piece_type: PieceType::Rook,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::White }),

            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,
            None, None, None, None, None, None, None, None,

            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Pawn,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Rook,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Queen,  color: Color::Black }),
            Some(Piece { piece_type: PieceType::King,   color: Color::Black }),
            Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
            Some(Piece { piece_type: PieceType::Rook,   color: Color::Black })
        ])
    }

    #[inline(always)]
    pub const fn get_piece(&self, square: Square) -> Option<&Piece> {
        self.0[square as usize].as_ref()
    }

    #[inline(always)]
    pub const fn set_piece(&mut self, square: Square, piece: Option<Piece>) {
        self.0[square as usize] = piece;
    }
}
