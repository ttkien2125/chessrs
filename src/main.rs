#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy)]
enum PieceType {
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
enum Color {
    White,
    Black,
}

#[derive(Clone, Copy)]
struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Clone, Copy)]
struct Square(Option<Piece>);

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(piece) = self.0 {
            let mut piece_name = format!("{}", piece.piece_type);

            if piece.color == Color::White {
                piece_name = piece_name.to_uppercase();
            }

            write!(f, "{}", piece_name)
        } else {
            write!(f, "-")
        }
    }
}

struct Board([[Square; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let board = [[Square(None); 8]; 8];
        Self(board)
    }

    pub fn from_fen(fen: &str) -> Self {
        let mut board = Board::new();

        let position = fen.split(" ").next().unwrap();

        let rows = position.split("/");
        for (rank, row) in rows.enumerate() {
            let mut file = 0;
            for c in row.chars() {
                if c.is_ascii_alphabetic() {
                    let piece_color = if c.is_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    };

                    let piece_name = c.to_ascii_lowercase();

                    let piece_type = match piece_name {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        _ => panic!("Unexpected piece type"),
                    };

                    let piece = Piece {
                        piece_type,
                        color: piece_color,
                    };
                    board[(rank, file)] = Square(Some(piece));
                } else if c.is_numeric() {
                    file += c.to_digit(10).unwrap() as usize - 1;
                } else {
                    panic!("Unexpected character in FEN string")
                }

                file += 1;
            }
        }

        board
    }

    pub fn print(&self) {
        print!("    ");
        for file in 'a'..='h' {
            print!("{}   ", file);
        }
        println!();

        for rank in 0..8 {
            print!("{} | ", rank + 1);
            for file in 0..8 {
                print!("{} | ", self[(rank, file)]);
            }
            print!("{}", rank + 1);
            println!();
        }

        print!("    ");
        for file in 'a'..='h' {
            print!("{}   ", file);
        }
        println!();
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Square;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Self {
        let board =
            Board::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2");

        Self {
            board,
            turn: Color::White,
        }
    }
}

fn main() {
    let game = Game::new();
    game.board.print();
}
