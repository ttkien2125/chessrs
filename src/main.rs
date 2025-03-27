#![allow(dead_code)]

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

// const VALID_PIECE_NAMES: [&str; 12] = ["p", "n", "b", "r", "q", "k", "P", "N", "B", "R", "Q", "K"];

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

struct Move {
    pub start: (usize, usize),
    pub end: (usize, usize),
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

    pub fn starting_pos() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn is_move_valid(&self, chess_move: &Move) -> bool {
        let Move { start, end } = chess_move;

        let start_piece = self[*start].0;
        let end_piece = self[*end].0;

        if let Some(start_piece) = start_piece {
            match start_piece.piece_type {
                PieceType::Rook => {
                    println!("Start: {:?} - End: {:?}", start, end);
                    if start.1 == end.1 {
                        let ranks = if start.0 < end.0 {
                            start.0 + 1..=end.0
                        } else {
                            end.0..=start.0 - 1
                        };
                        for rank in ranks {
                            let current_piece = self[(rank, start.1)].0;
                            if let Some(current_piece) = current_piece {
                                if current_piece.color == start_piece.color {
                                    return false;
                                } else {
                                    return rank == end.0;
                                }
                            } else {
                                continue;
                            }
                        }
                    } else if start.0 == end.0 {
                        let files = if start.1 < end.1 {
                            start.1 + 1..=end.1
                        } else {
                            end.1..=start.1 - 1
                        };
                        for file in files {
                            let current_piece = self[(start.0, file)].0;
                            if let Some(current_piece) = current_piece {
                                if current_piece.color == start_piece.color {
                                    return false;
                                } else {
                                    return file == end.1;
                                }
                            } else {
                                continue;
                            }
                        }
                    } else {
                        return false;
                    }

                    true
                }

                _ => unimplemented!(),
            }
        } else {
            false
        }
    }

    pub fn make_move(&mut self, chess_move: &Move) {
        if !self.is_move_valid(chess_move) {
            println!("Invalid move: Does not follow chess rules!");
            return;
        }

        self[chess_move.end].0 = self[chess_move.start].0;
        self[chess_move.start].0 = None;
    }

    pub fn print(&self) {
        println!("-----------------------------------------");
        print!("      ");
        for file in 'a'..='h' {
            print!("{}   ", file);
        }
        println!();

        for rank in 0..8 {
            print!("  {} | ", 8 - rank);
            for file in 0..8 {
                print!("{} | ", self[(rank, file)]);
            }
            print!("{}", 8 - rank);
            println!();
        }

        print!("      ");
        for file in 'a'..='h' {
            print!("{}   ", file);
        }
        println!();
        println!("-----------------------------------------");
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
        let mut board = Board::new();

        board[(4, 4)] = Square(Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }));

        board[(4, 1)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));

        board[(4, 6)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));

        board[(2, 4)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));

        board[(6, 4)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));

        Self {
            board,
            turn: Color::White,
        }
    }
}

fn pos_to_index(pos: &str) -> Option<(usize, usize)> {
    if pos.len() == 2 {
        let mut pos = pos.chars();
        let file = pos.next().unwrap();
        let rank = pos.next().unwrap().to_digit(10).unwrap() as usize;

        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some((8 - rank, file as usize - 'a' as usize))
        } else {
            None
        }
    } else {
        panic!("Invalid position")
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.board.print();

        println!("Make your move:");

        let mut chess_move = String::new();

        std::io::stdin()
            .read_line(&mut chess_move)
            .expect("Failed to read line");

        let chess_move = chess_move.trim();
        if chess_move.len() == 5 {
            let piece_name = &chess_move[..1];
            let start_square = &chess_move[1..3];
            let end_square = &chess_move[3..];

            let start = pos_to_index(start_square);
            let end = pos_to_index(end_square);

            if let (Some(start), Some(end)) = (start, end) {
                let start_piece = game.board[start].to_string();
                if piece_name != start_piece {
                    println!("Invalid move: Incorrect piece!");
                    continue;
                }

                let chess_move = Move { start, end };
                game.board.make_move(&chess_move);
            } else {
                println!("Invalid move: Out of bounds index");
                continue;
            }
        } else {
            println!("Invalid move: Wrong syntax!");
            continue;
        }
    }
}
