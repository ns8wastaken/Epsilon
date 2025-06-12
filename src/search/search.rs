use rand::Rng;
use super::{Board, Move, MoveGen, TTBound, TranspositionTable};

pub struct Search;

#[allow(unused)]
impl Search {
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

    fn _alphabeta(board: &mut Board, tt: &mut TranspositionTable, depth: u8, mut alpha: i16, beta: i16) -> i16 {
        if depth == 0 {
            return board.evaluate();
        }

        let original_alpha = alpha;
        let key = board.get_zobrist_hash();

        if let Some(entry) = tt.retrieve(key) {
            if entry.depth >= depth {
                match entry.bound {
                    TTBound::Exact => return entry.score,
                    TTBound::Lower => alpha = if alpha > entry.score { alpha } else { entry.score },
                    TTBound::Upper => {
                        if entry.score <= alpha {
                            return entry.score;
                        }
                    }
                }

                if alpha >= beta {
                    return entry.score;
                }
            }
        }

        let mut best_score = i16::MIN + 1;

        let moves = MoveGen::get_pseudolegal_moves(board);
        for mv in moves {
            board.make_move(&mv);

            if board.was_illegal_move() {
                board.revert_state();
                continue;
            }

            let score = -Self::_alphabeta(board, tt, depth - 1, -beta, -alpha);

            board.revert_state();

            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
            }

            if score >= beta {
                return best_score;
            }
        }

        let bound = if best_score <= original_alpha {
            TTBound::Upper
        } else if best_score >= beta {
            TTBound::Lower
        } else {
            TTBound::Exact
        };

        tt.store(key, depth, bound, best_score);

        best_score
    }

    pub fn alphabeta(board: &mut Board, tt: &mut TranspositionTable, depth: u8) -> Move {
        let mut best_score = i16::MIN + 1;
        let mut best_move: Option<Move> = None;

        let moves = MoveGen::get_pseudolegal_moves(board);
        for mv in moves {
            board.make_move(&mv);

            if board.was_illegal_move() {
                board.revert_state();
                continue;
            }

            let score = -Self::_alphabeta(board, tt, depth - 1, i16::MIN + 1, i16::MAX);

            board.revert_state();

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }
        }

        match best_move {
            Some(mv) => mv,
            None     => panic!(),
        }
    }
}
