use std::ops::{Index, IndexMut};

use crate::{Color, Move, Piece, PieceType, Square};

pub struct Board([[Square; 8]; 8]);

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

                _ => {
                    unimplemented!("{}", start_piece)
                }
            }
        } else {
            false
        }
    }

    pub fn make_move(&mut self, chess_move: &Move) {
        if !self.is_move_valid(chess_move) {
            println!("Invalid move!");
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
