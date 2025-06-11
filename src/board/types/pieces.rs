use super::color::Color;
use piece_macros::define_pieces;

define_pieces! {
    #[derive(Clone, Copy, Debug, PartialEq)],
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    #[inline(always)]
    pub const fn index(&self, color: &Color) -> usize {
        *self as usize + PIECETYPE_COUNT * color.index()
    }

    pub const fn value(&self) -> i16 {
        match self {
            PieceType::Pawn   => 100,
            PieceType::Knight => 300,
            PieceType::Bishop => 300,
            PieceType::Rook   => 500,
            PieceType::Queen  => 900,
            PieceType::King   => 10000,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub const fn new(piece_type: PieceType, color: Color) -> Self {
        Self { piece_type, color }
    }

    pub const fn from_char(c: char) -> Self {
        match c {
            'P' => Piece { piece_type: PieceType::Pawn,   color: Color::White },
            'p' => Piece { piece_type: PieceType::Pawn,   color: Color::Black },

            'N' => Piece { piece_type: PieceType::Knight, color: Color::White },
            'n' => Piece { piece_type: PieceType::Knight, color: Color::Black },

            'B' => Piece { piece_type: PieceType::Bishop, color: Color::White },
            'b' => Piece { piece_type: PieceType::Bishop, color: Color::Black },

            'R' => Piece { piece_type: PieceType::Rook,   color: Color::White },
            'r' => Piece { piece_type: PieceType::Rook,   color: Color::Black },

            'Q' => Piece { piece_type: PieceType::Queen,  color: Color::White },
            'q' => Piece { piece_type: PieceType::Queen,  color: Color::Black },

            'K' => Piece { piece_type: PieceType::King,   color: Color::White },
            'k' => Piece { piece_type: PieceType::King,   color: Color::Black },

            _ => unreachable!()
        }
    }

    pub const fn to_char(&self) -> char {
        match (self.piece_type, &self.color) {
            (PieceType::Pawn,   Color::White) => 'P',
            (PieceType::Pawn,   Color::Black) => 'p',

            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Knight, Color::Black) => 'n',

            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Bishop, Color::Black) => 'b',

            (PieceType::Rook,   Color::White) => 'R',
            (PieceType::Rook,   Color::Black) => 'r',

            (PieceType::Queen,  Color::White) => 'Q',
            (PieceType::Queen,  Color::Black) => 'q',

            (PieceType::King,   Color::White) => 'K',
            (PieceType::King,   Color::Black) => 'k',
        }
    }

    #[inline(always)]
    pub const fn index(&self) -> usize {
        self.piece_type as usize + PIECETYPE_COUNT * self.color.index()
    }
}
