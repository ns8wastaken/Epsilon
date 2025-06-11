pub type Square = u8;

pub trait SquareExt {
    const ALGEBRAIC_TABLE: [&'static str; 64] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
    ];

    fn from_algebraic(alg: &str) -> Self;
    fn to_algebraic(&self) -> &'static str;
}

impl SquareExt for Square {
    fn from_algebraic(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();
        assert!(chars.len() == 2, "Invalid algebraic square {}", s);

        let file = chars[0];
        let rank = chars[1];
        assert!(
            ('a'..='h').contains(&file) && ('1'..='8').contains(&rank),
            "Invalid algebraic square {}",
            s
        );

        (rank as u8 - b'1') * 8 + file as u8 - b'a'
    }

    fn to_algebraic(&self) -> &'static str {
        assert!((0u8..64u8).contains(&self));
        Self::ALGEBRAIC_TABLE[*self as usize]
    }
}
