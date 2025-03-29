use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn pawn(color: Color) -> Self {
        Self {
            piece_type: PieceType::Pawn,
            color,
        }
    }

    pub fn knight(color: Color) -> Self {
        Self {
            piece_type: PieceType::Knight,
            color,
        }
    }

    pub fn bishop(color: Color) -> Self {
        Self {
            piece_type: PieceType::Bishop,
            color,
        }
    }

    pub fn rook(color: Color) -> Self {
        Self {
            piece_type: PieceType::Rook,
            color,
        }
    }

    pub fn queen(color: Color) -> Self {
        Self {
            piece_type: PieceType::Queen,
            color,
        }
    }

    pub fn king(color: Color) -> Self {
        Self {
            piece_type: PieceType::King,
            color,
        }
    }

    pub fn simple_char(&self) -> char {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => 'P',
            (PieceType::Pawn, Color::Black) => 'p',
            (PieceType::Knight, Color::White) => 'N',
            (PieceType::Knight, Color::Black) => 'n',
            (PieceType::Bishop, Color::White) => 'B',
            (PieceType::Bishop, Color::Black) => 'b',
            (PieceType::Rook, Color::White) => 'R',
            (PieceType::Rook, Color::Black) => 'r',
            (PieceType::Queen, Color::White) => 'Q',
            (PieceType::Queen, Color::Black) => 'q',
            (PieceType::King, Color::White) => 'K',
            (PieceType::King, Color::Black) => 'k',
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'P' => Some(Self::pawn(Color::White)),
            'p' => Some(Self::pawn(Color::Black)),
            'N' => Some(Self::knight(Color::White)),
            'n' => Some(Self::knight(Color::Black)),
            'B' => Some(Self::bishop(Color::White)),
            'b' => Some(Self::bishop(Color::Black)),
            'R' => Some(Self::rook(Color::White)),
            'r' => Some(Self::rook(Color::Black)),
            'Q' => Some(Self::queen(Color::White)),
            'q' => Some(Self::queen(Color::Black)),
            'K' => Some(Self::king(Color::White)),
            'k' => Some(Self::king(Color::Black)),
            _ => None,
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_char())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square(pub Option<Piece>);

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(piece) = self.0 {
            write!(f, "{}", piece)
        } else {
            write!(f, "-")
        }
    }
}
