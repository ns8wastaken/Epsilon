use crate::movegen::MoveGen;

use super::commands::{UciCommand, DebugCommand};
use super::uci_io::UciIO;
use super::{
    Board,
    TranspositionTable,
    Debug,
    Search,
    Move,
    Square, SquareExt,
    BitboardExt
};

macro_rules! uci {
    ($cmd:ident) => {
        UciCommand::$cmd
    };
    (pos startpos $moves:expr) => {
        UciCommand::Position { fen: None, moves: $moves }
    };
    (pos fen $fen:ident $moves:expr) => {
        UciCommand::Position { fen: Some($fen), moves: $moves }
    };
    (unknown $ctx:expr) => {
        UciCommand::Unknown($ctx)
    };

    (debug $cmd:ident) => {
        UciCommand::Debug(DebugCommand::$cmd)
    };
    (debug $cmd:ident $ctx:expr) => {
        UciCommand::Debug(DebugCommand::$cmd($ctx))
    };
}

pub struct UciParser;

impl UciParser {
    fn parse(s: &str) -> UciCommand {
        let mut tokens = s
            .split_whitespace()
            .map(|x| x.trim());

        let s_string = s.to_string();

        match tokens.next() {
            Some("uci") => uci!(Uci),

            Some("isready") => uci!(IsReady),

            Some("stop") => uci!(Stop),

            Some("quit") => uci!(Quit),

            Some("position") => match tokens.next() {
                Some("startpos") => {
                    // Get moves
                    let mut moves = Vec::new();
                    if tokens.next() == Some("moves") {
                        while let Some(t) = tokens.next() {
                            moves.push(t.to_string());
                        }
                    }
                    uci!(pos startpos moves)
                }

                Some("fen") => {
                    // Get fen
                    let fen: Vec<&str> = tokens
                        .by_ref()
                        .take(6)
                        .collect();

                    if fen.len() < 4 {
                        panic!();
                    }

                    // Get moves
                    let mut moves = Vec::new();
                    if tokens.next() == Some("moves") {
                        while let Some(t) = tokens.next() {
                            moves.push(t.to_string());
                        }
                    }

                    uci!(pos fen fen moves)
                } // Some("fen")

                _ => uci!(unknown s_string),
            } // Some("position")

            Some("go") => uci!(Go),

            Some("debug") => match tokens.next() {
                Some("fen") => uci!(debug Fen),

                Some("print") => match tokens.next() {
                    Some("occupied") => uci!(debug PrintOccupied),

                    Some("attacks") => match tokens.next() {
                        Some(n) => uci!(debug PrintAttacks n.to_string()),
                        None    => uci!(debug Unknown s_string),
                    }

                    Some("moves") => match tokens.next() {
                        Some(n) => uci!(debug PrintMoves n.to_string()),
                        None    => uci!(debug Unknown s_string),
                    }

                    _ => uci!(debug Print)
                }

                Some("pos") => match tokens.next() {
                    Some(n) => uci!(debug Position n.to_string()),
                    None    => uci!(debug Unknown s_string),
                }

                Some("enpassant") => uci!(debug EnPassant),

                Some("move") => match tokens.next() {
                    Some(mv) => uci!(debug Move mv.to_string()),
                    None     => uci!(debug Unknown s_string),
                }

                Some("undo") => uci!(debug Undo),

                Some("castling") => uci!(debug CastlingRights),

                Some("perft") => match tokens.next() {
                    Some("singleline") => match tokens.next() {
                        Some(n) => uci!(debug PerftSingleLine n.parse().unwrap()),
                        None    => uci!(debug Unknown s_string),
                    }
                    Some(n) => uci!(debug Perft n.parse().unwrap()),
                    None    => uci!(debug Unknown s_string),
                },

                Some("divide") => match tokens.next() {
                    Some(n) => uci!(debug Divide n.parse().unwrap()),
                    None    => uci!(debug Unknown s_string),
                },

                Some("allstats") => uci!(debug AllStats),

                _ => uci!(debug Unknown s_string)
            }

            _ => uci!(unknown s_string),
        }
    }

    pub fn run_loop() {
        let mut io = UciIO::new();

        let mut board = Board::startpos();
        let mut tt = TranspositionTable::new(1 << 20);

        while let Some(input) = io.input() {
            if input.is_empty() {
                continue;
            }

            let command = Self::parse(&input);

            match command {
                UciCommand::Uci => {
                    io.out("id name Epsilon");
                    io.out("id author ns8");
                    io.out("uciok");
                }

                UciCommand::IsReady => {
                    io.out("readyok");
                }

                UciCommand::Position {
                    fen,
                    moves
                } => {
                    match fen {
                        Some(v) => board = Board::from_fen(v),
                        None    => board = Board::startpos(),
                    }

                    for mv in moves {
                        let mut mv = Move::from_uci(mv.as_str());
                        board.find_move_type(&mut mv);
                        board.make_move(&mv);
                    }
                }

                UciCommand::Go => {
                    // println!("bestmove {}", Search::random(&mut board, &mut rng).to_uci());
                    println!("bestmove {}", Search::alphabeta(&mut board, &mut tt, 5).to_uci());
                }

                UciCommand::Stop => break,

                UciCommand::Quit => break,

                UciCommand::Debug(command) => match command {
                    DebugCommand::Fen => io.outfmt(format_args!("{}", board.to_fen())),

                    DebugCommand::Print => board.print(),

                    DebugCommand::PrintOccupied => {
                        let occupied = board.get_occupied();
                        io.out("Black");
                        occupied.black.print();
                        io.out("White");
                        occupied.white.print();
                        io.out("All");
                        occupied.all.print();
                    }

                    DebugCommand::EnPassant => match board.get_en_passant_square() {
                        Some(n) => io.outfmt(format_args!("{}", n.to_algebraic())),
                        None    => io.out("None"),
                    }

                    DebugCommand::PrintAttacks(square) => {
                        let square = Square::from_algebraic(&square);
                        if let Some(piece) = board.get_piece(square) {
                            MoveGen::attacks(&board, &piece.piece_type, &piece.color, square).print();
                        }
                    }

                    DebugCommand::PrintMoves(square) => {
                        let square = Square::from_algebraic(&square);
                        if let Some(piece) = board.get_piece(square) {
                            MoveGen::moves(&board, &piece.piece_type, &piece.color, square).print();
                        }
                    }

                    DebugCommand::Position(pos) => match pos.as_str() {
                        "kiwipete" => board = Board::from_fen(
                            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
                                .split(' ')
                                .collect()
                        ),

                        _ => {}
                    }

                    DebugCommand::Move(mv) => {
                        let mut mv = Move::from_uci(mv.as_str());
                        board.find_move_type(&mut mv);
                        board.make_move(&mv);
                    },

                    DebugCommand::Undo => board.revert_state(),

                    DebugCommand::CastlingRights => {
                        io.outfmt(format_args!("{:#?}", board.get_castling_rights()));
                    }

                    DebugCommand::AllStats => {
                        io.out("------------------------------");
                        // Castling Rights
                        io.outfmt(format_args!("{:#?}\n", board.get_castling_rights()));
                        // Color to move
                        io.outfmt(format_args!("Color to move: {:#?}", board.color_to_move()));
                        // En passant square
                        io.outfmt(format_args!("En passant square: {:#?}", board.get_en_passant_square()));
                        io.out("------------------------------");
                    }

                    DebugCommand::Perft(depth) => {
                        let (elapsed, nodes) = Debug::timed_perft(&mut board, depth);
                        io.outfmt(format_args!(
                            "Time: {elapsed}s\nNodes: {nodes}\nNodes/s: {}",
                            (nodes as f64 / elapsed) as u64
                        ));
                    }

                    DebugCommand::PerftSingleLine(depth) => {
                        let (elapsed, nodes) = Debug::timed_perft(&mut board, depth);
                        io.outfmt(format_args!(
                            "{nodes}; {elapsed}; {}",
                            (nodes as f64 / elapsed) as u64
                        ));
                    }

                    DebugCommand::Divide(depth) => {
                        io.outfmt(format_args!("\n{}", Debug::divide(&mut board, depth)));
                    }

                    DebugCommand::Unknown(cmd) => {
                        io.outfmt(format_args!("info string Unknown debug command: {cmd}"))
                    }
                }

                UciCommand::Unknown(cmd) => {
                    io.outfmt(format_args!("info string Unknown command: {cmd}"));
                }
            }
        }
    }
}
