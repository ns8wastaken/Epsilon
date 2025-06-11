use super::{PieceType, Square, SquareExt};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MoveType {
    Unknown,
    Quiet,
    Capture,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    Promotion(PieceType),
}

#[derive(Clone, Copy, Debug)]
pub struct Move {
    pub from: Square,
    pub to: Square,
    pub move_type: MoveType,
}

impl Move {
    pub const fn new(
        from: Square,
        to: Square,
        move_type: MoveType
    ) -> Self {
        Self { from, to, move_type }
    }

    pub fn from_uci(uci: &str) -> Move {
        let from = Square::from_algebraic(&uci[0..2]);
        let to = Square::from_algebraic(&uci[2..4]);

        let move_type = match uci.chars().nth(4) {
            Some('n') | Some('N') => MoveType::Promotion(PieceType::Knight),
            Some('b') | Some('B') => MoveType::Promotion(PieceType::Bishop),
            Some('r') | Some('R') => MoveType::Promotion(PieceType::Rook),
            Some('q') | Some('Q') => MoveType::Promotion(PieceType::Queen),
            None => MoveType::Unknown,
            _ => panic!(),
        };

        Self { from, to, move_type }
    }

    pub fn to_uci(&self) -> String {
        let mut uci = format!("{}{}", self.from.to_algebraic(), self.to.to_algebraic());

        if let MoveType::Promotion(promotion) = self.move_type {
            let promo_char = match promotion {
                PieceType::Knight => 'n',
                PieceType::Bishop => 'b',
                PieceType::Rook   => 'r',
                PieceType::Queen  => 'q',
                _ => unreachable!("Invalid promotion piece"),
            };
            uci.push(promo_char);
        }

        uci
    }
}
