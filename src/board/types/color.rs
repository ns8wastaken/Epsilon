#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub const fn inverse(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    pub const fn from_bool(bool: bool) -> Self {
        match bool {
            true  => Self::White,
            false => Self::Black,
        }
    }

    pub const fn is_white(&self) -> bool {
        match self {
            Self::White => true,
            Self::Black => false,
        }
    }

    pub const fn index(&self) -> usize {
        match self {
            Self::White => 0,
            Self::Black => 1,
        }
    }
}
