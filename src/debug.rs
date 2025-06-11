use std::time::Instant;
use crate::{movegen::MoveGen, board::Board};

pub struct Debug;

impl Debug {
    pub fn perft(board: &mut Board, depth: i32) -> u64 {
        if depth <= 0 {
            return 1;
        }

        let mut total_nodes: u64 = 0;

        let move_list = MoveGen::get_pseudolegal_moves(board);
        for mv in &move_list {
            board.make_move(mv);

            if board.was_illegal_move() {
                board.revert_state();
                continue;
            }

            total_nodes += Self::perft(board, depth - 1);

            board.revert_state();
        }

        return total_nodes;
    }

    pub fn divide(board: &mut Board, depth: i32) -> u64 {
        let mut total_nodes: u64 = 0;

        let move_list = MoveGen::get_pseudolegal_moves(board);
        for mv in &move_list {
            board.make_move(mv);

            if board.was_illegal_move() {
                board.revert_state();
                continue;
            }

            let move_nodes = Self::perft(board, depth - 1);
            println!("{}: {move_nodes}", mv.to_uci());
            total_nodes += move_nodes;

            board.revert_state();
        }

        return total_nodes;
    }

    pub fn timed_perft(board: &mut Board, depth: i32) -> (f64, u64) {
        let start = Instant::now();
        let nodes = Self::perft(board, depth);
        let elapsed = start.elapsed().as_secs_f64();
        (elapsed, nodes)
    }
}
