use std::fmt::Display;

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn index(&self) -> usize {
        match self {
            Self::White => 1,
            Self::Black => 2,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            1 => Some(Self::White),
            2 => Some(Self::Black),

            _ => None,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::White => write!(f, "White"),
            Color::Black => write!(f, "Black"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,

    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
}

impl Piece {
    pub fn index(&self) -> usize {
        match self {
            Self::WhitePawn => 0,
            Self::WhiteKnight => 1,
            Self::WhiteBishop => 2,
            Self::WhiteRook => 3,
            Self::WhiteQueen => 4,
            Self::WhiteKing => 5,

            Self::BlackPawn => 6,
            Self::BlackKnight => 7,
            Self::BlackBishop => 8,
            Self::BlackRook => 9,
            Self::BlackQueen => 10,
            Self::BlackKing => 11,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::WhitePawn),
            1 => Some(Self::WhiteKnight),
            2 => Some(Self::WhiteBishop),
            3 => Some(Self::WhiteRook),
            4 => Some(Self::WhiteQueen),
            5 => Some(Self::WhiteKing),

            6 => Some(Self::BlackPawn),
            7 => Some(Self::BlackKnight),
            8 => Some(Self::BlackBishop),
            9 => Some(Self::BlackRook),
            10 => Some(Self::BlackQueen),
            11 => Some(Self::BlackKing),

            _ => None,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::WhitePawn => Color::White,
            Self::WhiteKnight => Color::White,
            Self::WhiteBishop => Color::White,
            Self::WhiteRook => Color::White,
            Self::WhiteQueen => Color::White,
            Self::WhiteKing => Color::White,

            Self::BlackPawn => Color::Black,
            Self::BlackKnight => Color::Black,
            Self::BlackBishop => Color::Black,
            Self::BlackRook => Color::Black,
            Self::BlackQueen => Color::Black,
            Self::BlackKing => Color::Black,
        }
    }

    pub fn simple_char(&self) -> char {
        match self {
            Self::WhitePawn => 'P',
            Self::WhiteKnight => 'N',
            Self::WhiteBishop => 'B',
            Self::WhiteRook => 'R',
            Self::WhiteQueen => 'Q',
            Self::WhiteKing => 'K',

            Self::BlackPawn => 'p',
            Self::BlackKnight => 'n',
            Self::BlackBishop => 'b',
            Self::BlackRook => 'r',
            Self::BlackQueen => 'q',
            Self::BlackKing => 'k',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'P' => Some(Self::WhitePawn),
            'N' => Some(Self::WhiteKnight),
            'B' => Some(Self::WhiteBishop),
            'R' => Some(Self::WhiteRook),
            'Q' => Some(Self::WhiteQueen),
            'K' => Some(Self::WhiteKing),

            'p' => Some(Self::BlackPawn),
            'n' => Some(Self::BlackKnight),
            'b' => Some(Self::BlackBishop),
            'r' => Some(Self::BlackRook),
            'q' => Some(Self::BlackQueen),
            'k' => Some(Self::BlackKing),

            _ => None,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_char())
    }
}
