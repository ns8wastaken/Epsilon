use rand::Rng;
use super::{Board, Move, MoveGen};

pub struct Engine;

impl Engine {
    pub fn random(board: &mut Board, rng: &mut rand::rngs::ThreadRng) -> Move {
        let mut legal_moves = Vec::new();

        let moves = MoveGen::get_pseudolegal_moves(board);

        for mv in moves {
            board.make_move(&mv);

            if board.was_illegal_move() {
                board.revert_state();
                continue;
            }

            legal_moves.push(mv);

            board.revert_state();
        }

        legal_moves[rng.random_range(0..legal_moves.len())]
    }

    pub fn 
}
