pub enum UciCommand<'a> {
    Uci,
    IsReady,
    Position {
        // Assume that the position is startpos if None
        fen: Option<Vec<&'a str>>,
        moves: Vec<String>
    },
    Go,
    Stop,
    Quit,
    Debug(DebugCommand),
    Unknown(String),
}

pub enum DebugCommand {
    Move(String),
    Undo,
    Fen,
    Print,
    PrintOccupied,
    PrintAttacks(String),
    PrintMoves(String),
    Position(String),
    EnPassant,
    // Eval,
    CastlingRights,
    AllStats,
    Perft(i32),
    PerftSingleLine(i32),
    Divide(i32),
    Unknown(String),
}
