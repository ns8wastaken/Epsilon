use crossterm::style::{self, Stylize};

use super::{
    Bitboard,
    CastlingRights,
    Color,
    HistoryState,
    Mailbox,
    MoveType, Move,
    Occupied,
    Piece, PieceType, PIECETYPE_COUNT,
    MoveGen,
    Square, SquareExt,
    Zobrist
};

pub struct Board {
    bitboards: [Bitboard; PIECETYPE_COUNT * 2],
    mailbox: Mailbox,
    occupied: Occupied,
    color_to_move: Color,
    en_passant_square: Option<Square>,
    castling_rights: CastlingRights,
    history: Vec<HistoryState>,
    zobrist: Zobrist,
}

impl Board {
    pub fn startpos() -> Self {
        let mut bitboards = [0; 12];

        bitboards[Piece::new(PieceType::Pawn,   Color::White).index()] = 0xff00;
        bitboards[Piece::new(PieceType::Knight, Color::White).index()] = 0x42;
        bitboards[Piece::new(PieceType::Bishop, Color::White).index()] = 0x24;
        bitboards[Piece::new(PieceType::Rook,   Color::White).index()] = 0x81;
        bitboards[Piece::new(PieceType::Queen,  Color::White).index()] = 0x8;
        bitboards[Piece::new(PieceType::King,   Color::White).index()] = 0x10;

        bitboards[Piece::new(PieceType::Pawn,   Color::Black).index()] = 0xff000000000000;
        bitboards[Piece::new(PieceType::Knight, Color::Black).index()] = 0x4200000000000000;
        bitboards[Piece::new(PieceType::Bishop, Color::Black).index()] = 0x2400000000000000;
        bitboards[Piece::new(PieceType::Rook,   Color::Black).index()] = 0x8100000000000000;
        bitboards[Piece::new(PieceType::Queen,  Color::Black).index()] = 0x800000000000000;
        bitboards[Piece::new(PieceType::King,   Color::Black).index()] = 0x1000000000000000;

        Self {
            bitboards,
            mailbox: Mailbox::startpos(),
            occupied: Occupied {
                white: 0xffff,
                black: 0xffff000000000000,
                all: 0xffff00000000ffff
            },
            color_to_move: Color::White,
            en_passant_square: None,
            castling_rights: CastlingRights::default(),
            history: Vec::new(),
            zobrist: Zobrist::new()
        }
    }

    pub fn from_fen(split_fen: Vec<&str>) -> Self {
        let mut bitboards: [Bitboard; 12] = [0; 12];
        let mut mailbox = Mailbox::new();
        let mut occupied = Occupied::new();

        let mut square: Square = 56; // Start from the top-left corner (a8)

        for c in split_fen[0].chars() {
            if c == '/' {
                square -= 16; // Move to the start of the next rank
            }
            else if c.is_ascii_digit() {
                square += c as u8 - '0' as u8;
            }
            else {
                let piece = Piece::from_char(c);

                bitboards[piece.index()] |= 1 << square;
                mailbox.set_piece(square, Some(piece));

                match piece.color {
                    Color::White => occupied.white |= 1 << square,
                    Color::Black => occupied.black |= 1 << square,
                }

                square += 1;
            }
        }

        occupied.all = occupied.white | occupied.black;

        let color_to_move = Color::from_bool(split_fen[1] == "w");

        let mut castling_rights = CastlingRights::default_but_false();
        for c in split_fen[2].chars() {
            match c {
                'K' => castling_rights.white_king_side  = true,
                'Q' => castling_rights.white_queen_side = true,
                'k' => castling_rights.black_king_side  = true,
                'q' => castling_rights.black_queen_side = true,

                _ => {}
            }
        }


        let en_passant_square = if split_fen[3] == "-" {
            None
        } else {
            Some(Square::from_algebraic(&split_fen[3]))
        };

        // TODO: Also read the ply and move count

        Self {
            bitboards,
            mailbox,
            occupied,
            color_to_move,
            en_passant_square,
            castling_rights,
            history: Vec::new(),
            zobrist: Zobrist::new()
        }
    }

    pub fn to_fen(&self) -> String {
        let mut result = String::new();

        let mut square = 56;
        while square < 64 {
            let mut empty_count = 0;

            for _ in 0..8 {
                match self.mailbox.get_piece(square) {
                    Some(piece) => {
                        if empty_count != 0 {
                            result.push_str(empty_count.to_string().as_str());
                        }

                        result.push(piece.to_char());
                        empty_count = 0;
                    }

                    None => empty_count += 1,
                }

                square += 1;
            }

            if empty_count != 0 {
                result.push_str(empty_count.to_string().as_str());
            }

            result.push('/');
            square = square.wrapping_sub(16);
        }

        result.pop();

        result.push(' ');
        result.push(if self.color_to_move == Color::White { 'w' } else { 'b' });
        result.push(' ');

        if self.castling_rights.white_king_side {
            result.push('K');
        }
        if self.castling_rights.white_queen_side {
            result.push('Q');
        }
        if self.castling_rights.black_king_side {
            result.push('k');
        }
        if self.castling_rights.black_queen_side {
            result.push('q');
        }
        if result.chars().last().unwrap() == ' ' {
            result.push('-');
        }

        result.push(' ');

        match self.en_passant_square {
            Some(square) => {
                result.push('0');
                result.push_str(square.to_string().as_str());
                result.push(' ');
            }
            None => {
                result.push('-');
                result.push(' ');
            }
        }

        // result.push_str((self.ply_count / 2).to_string().as_str());
        // result.push(' ');
        // result.push_str(self.ply_count.to_string().as_str());

        result
    }

    pub fn print(&self) {
        println!("  +-----------------+");

        for rank in (0..8).rev() {
            print!("{} |", rank + 1);
            for file in 0..8 {
                let idx = rank * 8 + file;
                let square = self.mailbox.get_piece(idx);
                let symbol = match square {
                    Some(piece) => {
                        let chr = piece.to_char();
                        if chr.is_uppercase() {
                            chr.with(style::Color::Red)
                        } else {
                            chr.with(style::Color::Blue)
                        }
                    },
                    None => '.'.stylize(),
                };
                if file == 0 { print!(" "); }
                print!("{} ", symbol);
            }
            println!("|");
        }

        println!("  +-----------------+");
        println!("    a b c d e f g h");
    }

    #[inline(always)]
    pub const fn get_piece(&self, square: Square) -> Option<&Piece> {
        self.mailbox.get_piece(square)
    }

    #[inline(always)]
    pub const fn get_occupied(&self) -> &Occupied {
        &self.occupied
    }

    #[inline(always)]
    pub const fn get_castling_rights(&self) -> &CastlingRights {
        &self.castling_rights
    }

    #[inline(always)]
    pub const fn get_en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    #[inline(always)]
    pub const fn color_to_move(&self) -> &Color {
        &self.color_to_move
    }

    pub const fn place_piece(&mut self, square: Square, piece: Piece) {
        self.bitboards[piece.index()] |= 1u64 << square;
        self.mailbox.set_piece(square, Some(piece))
    }

    pub const fn remove_piece(&mut self, square: Square, piece: &Piece) {
        self.bitboards[piece.index()] &= !(1u64 << square);
        self.mailbox.set_piece(square, None);
    }

    pub fn find_move_type(&self, mv: &mut Move) {
        if mv.move_type != MoveType::Unknown { return; }

        let piece = self.mailbox.get_piece(mv.from).copied().unwrap();
        let target = self.mailbox.get_piece(mv.to).copied();

        // Detect castling
        if piece.piece_type == PieceType::King {
            match (piece.color, mv.from, mv.to) {
                (Color::White, 4,  6)  => {
                    mv.move_type = MoveType::CastleKingside;
                    return;
                }
                (Color::White, 4,  2)  => {
                    mv.move_type = MoveType::CastleQueenside;
                    return;
                }
                (Color::Black, 60, 62) => {
                    mv.move_type = MoveType::CastleKingside;
                    return;
                }
                (Color::Black, 60, 58) => {
                    mv.move_type = MoveType::CastleQueenside;
                    return;
                }
                _ => {}
            }
        }

        // Detect en passant
        if piece.piece_type == PieceType::Pawn {
            if let Some(en_passant_square) = self.en_passant_square {
                if mv.to == en_passant_square {
                    mv.move_type = MoveType::EnPassant;
                    return;
                }
            }
        }

        // Capture vs quiet
        mv.move_type = if target.is_some() {
            MoveType::Capture
        } else {
            MoveType::Quiet
        };
    }

    pub fn save_state(&mut self) {
        self.history.push(HistoryState {
            bitboards:         self.bitboards,
            mailbox:           self.mailbox,
            occupied:          self.occupied,
            color_to_move:     self.color_to_move,
            en_passant_square: self.en_passant_square,
            castling_rights:   self.castling_rights
        });
    }

    fn update_occupied(&mut self) {
        self.occupied.white = {
            let mut occ = 0;
            for i in 0..PIECETYPE_COUNT {
                occ |= self.bitboards[i];
            }
            occ
        };
        self.occupied.black = {
            let mut occ = 0;
            for i in PIECETYPE_COUNT..self.bitboards.len() {
                occ |= self.bitboards[i];
            }
            occ
        };
        self.occupied.all = self.occupied.white | self.occupied.black;
    }

    pub fn revert_state(&mut self) {
        let state = self.history.pop().unwrap();

        self.bitboards         = state.bitboards;
        self.mailbox           = state.mailbox;
        self.occupied          = state.occupied;
        // TODO: Try to swap color manually instead of saving and reverting it
        self.color_to_move     = state.color_to_move;
        self.en_passant_square = state.en_passant_square;
        self.castling_rights   = state.castling_rights;
    }

    pub fn make_move(&mut self, mv: &Move) {
        // Save in the annals of time
        self.save_state();

        let moving_piece = *self.mailbox
            .get_piece(mv.from)
            .unwrap();

        let is_white = moving_piece.color.is_white();

        // Handle castling rights
        match moving_piece.piece_type {
            // If the king moves, remove his rights
            PieceType::King => match moving_piece.color {
                Color::White => {
                    self.castling_rights.white_king_side  = false;
                    self.castling_rights.white_queen_side = false;
                }
                Color::Black => {
                    self.castling_rights.black_king_side  = false;
                    self.castling_rights.black_queen_side = false;
                }
            },
            // If a rook moves, remove the kings' rights too
            PieceType::Rook => match mv.from {
                0  => self.castling_rights.white_queen_side = false,
                7  => self.castling_rights.white_king_side  = false,
                56 => self.castling_rights.black_queen_side = false,
                63 => self.castling_rights.black_king_side  = false,
                _ => {}
            }
            _ => {}
        }

        // If a piece moves to one of the corners, the king is dethroned
        match mv.to {
            0  => self.castling_rights.white_queen_side = false,
            7  => self.castling_rights.white_king_side  = false,
            56 => self.castling_rights.black_queen_side = false,
            63 => self.castling_rights.black_king_side  = false,
            _ => {}
        }

        match mv.move_type {
            MoveType::Quiet => {}

            MoveType::Capture => {
                let Some(&captured_piece) = self.mailbox.get_piece(mv.to) else { panic!() };
                self.remove_piece(mv.to, &captured_piece);
            }

            MoveType::CastleKingside => match moving_piece.color {
                Color::White => {
                    self.castling_rights.white_king_side = false;
                    let Some(&rook) = self.mailbox.get_piece(7) else { panic!() };
                    self.remove_piece(7, &rook);
                    self.place_piece(5, rook);
                }
                Color::Black => {
                    self.castling_rights.black_king_side = false;
                    let Some(&rook) = self.mailbox.get_piece(63) else { panic!() };
                    self.remove_piece(63, &rook);
                    self.place_piece(61, rook);
                }
            }

            MoveType::CastleQueenside => match moving_piece.color {
                Color::White => {
                    self.castling_rights.white_queen_side = false;
                    let Some(&rook) = self.mailbox.get_piece(0) else { panic!() };
                    self.remove_piece(0, &rook);
                    self.place_piece(3, rook);
                }
                Color::Black => {
                    self.castling_rights.black_queen_side = false;
                    let Some(&rook) = self.mailbox.get_piece(56) else { panic!() };
                    self.remove_piece(56, &rook);
                    self.place_piece(59, rook);
                }
            }

            MoveType::EnPassant => {
                let captured_pawn_square = if is_white { mv.to - 8 } else { mv.to + 8 };
                let Some(&captured_pawn) = self.mailbox.get_piece(captured_pawn_square) else { panic!() };
                self.remove_piece(captured_pawn_square, &captured_pawn);
            }

            MoveType::Promotion(promotion) => {
                self.remove_piece(mv.from, &moving_piece);

                // Remove any captured piece at the destination
                if let Some(&captured_piece) = self.mailbox.get_piece(mv.to) {
                    self.remove_piece(mv.to, &captured_piece);
                }

                self.place_piece(mv.to, Piece {
                    piece_type: promotion,
                    color: moving_piece.color,
                });

                // Return to avoid placing a pawn (because promotion)
                self.color_to_move = self.color_to_move.inverse();
                self.en_passant_square = None;
                self.update_occupied();
                return;
            }

            MoveType::Unknown => panic!(),
        }

        self.remove_piece(mv.from, &moving_piece);
        self.place_piece(mv.to, moving_piece);

        // Flip color
        self.color_to_move = self.color_to_move.inverse();

        // Clear en passant square
        self.en_passant_square = None;

        // Set the en passant square for the next turn
        // TODO: Make a MoveType for DoublePawnPush
        if moving_piece.piece_type == PieceType::Pawn {
            if is_white && (mv.to == mv.from + 16) {
                self.en_passant_square = Some(mv.from + 8);
            }
            else if !is_white && (mv.to == mv.from.wrapping_sub(16)) {
                self.en_passant_square = Some(mv.from - 8);
            }
        }

        self.update_occupied();
    }

    pub fn is_attacked(&self, square: Square) -> bool {
        let enemy_color   = self.color_to_move.inverse();
        let enemy_pawns   = self.bitboards[PieceType::Pawn.index(&enemy_color)];
        let enemy_knights = self.bitboards[PieceType::Knight.index(&enemy_color)];
        let enemy_bishops = self.bitboards[PieceType::Bishop.index(&enemy_color)];
        let enemy_rooks   = self.bitboards[PieceType::Rook.index(&enemy_color)];
        let enemy_queens  = self.bitboards[PieceType::Queen.index(&enemy_color)];
        let enemy_king    = self.bitboards[PieceType::King.index(&enemy_color)];

        (
            MoveGen::attacks(self, &PieceType::Pawn, &self.color_to_move, square) & enemy_pawns
            | MoveGen::attacks(self, &PieceType::Knight, &self.color_to_move, square) & enemy_knights
            | MoveGen::attacks(self, &PieceType::Bishop, &self.color_to_move, square) & enemy_bishops
            | MoveGen::attacks(self, &PieceType::Rook, &self.color_to_move, square) & enemy_rooks
            | MoveGen::attacks(self, &PieceType::Queen, &self.color_to_move, square) & enemy_queens
            | MoveGen::attacks(self, &PieceType::King, &self.color_to_move, square) & enemy_king
        ) != 0
    }

    pub fn can_castle_kingside(&self) -> bool {
        if ((self.color_to_move == Color::White) && !self.castling_rights.white_king_side)
            || ((self.color_to_move == Color::Black) && !self.castling_rights.black_king_side)
        {
            return false;
        }

        let squares =
            if self.color_to_move == Color::White {
                (4, 5, 6)
            } else {
                (60, 61, 62)
            };

        if self.mailbox.get_piece(squares.1).is_some()
            || self.mailbox.get_piece(squares.2).is_some()
        {
            return false;
        }

        if self.is_attacked(squares.0)
            || self.is_attacked(squares.1)
            || self.is_attacked(squares.2)
        {
            return false;
        }

        true
    }

    pub fn can_castle_queenside(&self) -> bool {
        if ((self.color_to_move == Color::White) && !self.castling_rights.white_queen_side)
            || ((self.color_to_move == Color::Black) && !self.castling_rights.black_queen_side)
        {
            return false;
        }

        let squares =
            if self.color_to_move == Color::White {
                (4, 3, 2, 1)
            } else {
                (60, 59, 58, 57)
            };

        if self.mailbox.get_piece(squares.1).is_some()
            || self.mailbox.get_piece(squares.2).is_some()
            || self.mailbox.get_piece(squares.3).is_some()
        {
            return false;
        }

        if self.is_attacked(squares.0)
            || self.is_attacked(squares.1)
            || self.is_attacked(squares.2)
        {
            return false;
        }

        true
    }

    pub fn was_illegal_move(&mut self) -> bool {
        self.color_to_move = self.color_to_move.inverse();

        if self.is_attacked(self.bitboards[
            PieceType::King.index(&self.color_to_move)
        ].trailing_zeros() as u8) {
            return true;
        }

        self.color_to_move = self.color_to_move.inverse();

        false
    }

    pub fn evaluate(&self) -> i16 {
        let mut white_score = 0;
        let mut black_score = 0;

        for square in 0..64 {
            let Some(piece) = self.get_piece(square) else { continue; };

            let value = piece.piece_type.value();
            if piece.color == Color::White {
                white_score += value;
            } else {
                black_score += value;
            }
        }

        if self.color_to_move == Color::White {
            return white_score - black_score;
        } else {
            return black_score - white_score;
        }
    }

    pub fn get_zobrist_hash(&self) -> u64 {
        let mut hash = 0;

        // Hash pieces
        for square in 0..64 {
            if let Some(piece) = self.get_piece(square) {
                hash ^= self.zobrist.pieces[Zobrist::index(
                    piece.color.index(),
                    piece.piece_type as usize,
                    square
                )];
            }
        }

        // Hash side to move
        if self.color_to_move == Color::White {
            hash ^= self.zobrist.side_to_move;
        }

        // Hash castling rights
        let castling_rights = &self.castling_rights;
        if castling_rights.white_king_side  { hash ^= self.zobrist.castling_rights[0]; }
        if castling_rights.white_queen_side { hash ^= self.zobrist.castling_rights[1]; }
        if castling_rights.black_king_side  { hash ^= self.zobrist.castling_rights[2]; }
        if castling_rights.black_queen_side { hash ^= self.zobrist.castling_rights[3]; }

        // Hash en passant
        if let Some(ep_square) = self.en_passant_square {
            let file = ep_square % 8;
            hash ^= self.zobrist.en_passant_file[file as usize];
        }

        hash
    }
}
