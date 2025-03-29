use std::fmt::Display;

#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn simple_char(&self) -> &'static str {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::White) => "P",
            (PieceType::Pawn, Color::Black) => "p",
            (PieceType::Knight, Color::White) => "N",
            (PieceType::Knight, Color::Black) => "n",
            (PieceType::Bishop, Color::White) => "B",
            (PieceType::Bishop, Color::Black) => "b",
            (PieceType::Rook, Color::White) => "R",
            (PieceType::Rook, Color::Black) => "r",
            (PieceType::Queen, Color::White) => "Q",
            (PieceType::Queen, Color::Black) => "q",
            (PieceType::King, Color::White) => "K",
            (PieceType::King, Color::Black) => "k",
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        let (piece_type, color) = match c {
            'P' => (PieceType::Pawn, Color::White),
            'p' => (PieceType::Pawn, Color::Black),
            'N' => (PieceType::Knight, Color::White),
            'n' => (PieceType::Knight, Color::Black),
            'B' => (PieceType::Bishop, Color::White),
            'b' => (PieceType::Bishop, Color::Black),
            'R' => (PieceType::Rook, Color::White),
            'r' => (PieceType::Rook, Color::Black),
            'Q' => (PieceType::Queen, Color::White),
            'q' => (PieceType::Queen, Color::Black),
            'K' => (PieceType::King, Color::White),
            'k' => (PieceType::King, Color::Black),
            _ => return None,
        };

        Some(Self { piece_type, color })
    }

    // pub fn fancy_char(&self) -> &'static str {
    //     match self.piece_type {
    //         PieceType::Pawn => "♟︎",
    //         PieceType::Knight => "♞",
    //         PieceType::Bishop => "♝",
    //         PieceType::Rook => "♜",
    //         PieceType::Queen => "♛",
    //         PieceType::King => "♚",
    //     }
    // }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.simple_char())
    }
}

#[derive(Clone, Copy)]
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
