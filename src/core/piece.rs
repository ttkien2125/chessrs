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

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_name = match self {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };
        write!(f, "{}", piece_name)
    }
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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut piece_name = format!("{}", self.piece_type);

        if self.color == Color::White {
            piece_name = piece_name.to_uppercase();
        }

        write!(f, "{}", piece_name)
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
