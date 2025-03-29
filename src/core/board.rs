use std::ops::{Index, IndexMut};

use crate::{Move, Piece, PieceType, Square};

pub const EMPTY_FEN_STRING: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";
pub const STARTING_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Board([[Square; 8]; 8]);

impl Board {
    pub fn new() -> Self {
        let board = [[Square(None); 8]; 8];
        Self(board)
    }

    // TODO: Support the rest of the FEN string.
    pub fn from_fen(fen: &str) -> Option<Self> {
        let mut board = Board::new();

        let fen_config = fen.split(" ").collect::<Vec<_>>();
        if fen_config.len() != 6 {
            return None;
        }

        let position = fen_config[0];

        let rows = position.split("/");
        for (rank, row) in rows.enumerate() {
            let mut file = 0;
            for c in row.chars() {
                if c.is_ascii_alphabetic() {
                    match Piece::from_char(c) {
                        Some(piece) => board[(rank, file)] = Square(Some(piece)),
                        None => return None,
                    }
                } else if ('1'..='8').contains(&c) {
                    file += c.to_digit(10).unwrap() as usize - 1;
                } else {
                    return None;
                }

                file += 1;
            }

            if rank >= 8 {
                return None;
            }
        }

        Some(board)
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

#[cfg(test)]
mod tests {
    use crate::Color;

    use super::*;

    #[test]
    fn empty_fen() {
        let board = Board::from_fen(EMPTY_FEN_STRING).unwrap();
        for rank in 0..8 {
            for file in 0..8 {
                assert_eq!(board[(rank, file)], Square(None));
            }
        }
    }

    #[test]
    fn starting_fen() {
        let board = Board::from_fen(STARTING_FEN_STRING).unwrap();

        assert_eq!(board[(0, 0)], Square(Some(Piece::rook(Color::Black))));
        assert_eq!(board[(0, 1)], Square(Some(Piece::knight(Color::Black))));
        assert_eq!(board[(0, 2)], Square(Some(Piece::bishop(Color::Black))));
        assert_eq!(board[(0, 3)], Square(Some(Piece::queen(Color::Black))));
        assert_eq!(board[(0, 4)], Square(Some(Piece::king(Color::Black))));
        assert_eq!(board[(0, 5)], Square(Some(Piece::bishop(Color::Black))));
        assert_eq!(board[(0, 6)], Square(Some(Piece::knight(Color::Black))));
        assert_eq!(board[(0, 7)], Square(Some(Piece::rook(Color::Black))));

        for file in 0..8 {
            assert_eq!(board[(1, file)], Square(Some(Piece::pawn(Color::Black))));
            assert_eq!(board[(6, file)], Square(Some(Piece::pawn(Color::White))));
        }

        for rank in 2..6 {
            for file in 0..8 {
                assert_eq!(board[(rank, file)], Square(None));
            }
        }

        assert_eq!(board[(7, 0)], Square(Some(Piece::rook(Color::White))));
        assert_eq!(board[(7, 1)], Square(Some(Piece::knight(Color::White))));
        assert_eq!(board[(7, 2)], Square(Some(Piece::bishop(Color::White))));
        assert_eq!(board[(7, 3)], Square(Some(Piece::queen(Color::White))));
        assert_eq!(board[(7, 4)], Square(Some(Piece::king(Color::White))));
        assert_eq!(board[(7, 5)], Square(Some(Piece::bishop(Color::White))));
        assert_eq!(board[(7, 6)], Square(Some(Piece::knight(Color::White))));
        assert_eq!(board[(7, 7)], Square(Some(Piece::rook(Color::White))));
    }

    #[test]
    fn random_fen() {
        let board =
            Board::from_fen("r2q1rk1/2p1bppp/p1n1bn2/1p2p3/4P3/2P2N2/PPBN1PPP/R1BQR1K1 w - - 1 12")
                .unwrap();

        assert_eq!(board[(0, 0)], Square(Some(Piece::rook(Color::Black))));
        assert_eq!(board[(0, 3)], Square(Some(Piece::queen(Color::Black))));
        assert_eq!(board[(0, 6)], Square(Some(Piece::king(Color::Black))));

        assert_eq!(board[(1, 4)], Square(Some(Piece::bishop(Color::Black))));

        assert_eq!(board[(2, 2)], Square(Some(Piece::knight(Color::Black))));
        assert_eq!(board[(2, 5)], Square(Some(Piece::knight(Color::Black))));

        assert_eq!(board[(6, 2)], Square(Some(Piece::bishop(Color::White))));

        assert_eq!(board[(7, 0)], Square(Some(Piece::rook(Color::White))));
        assert_eq!(board[(7, 3)], Square(Some(Piece::queen(Color::White))));
        assert_eq!(board[(7, 6)], Square(Some(Piece::king(Color::White))));
    }

    #[test]
    #[should_panic]
    fn invalid_fen() {
        Board::from_fen("invalid fen string").unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_fen_wrong_char() {
        Board::from_fen("Hnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_fen_too_many_chars() {
        Board::from_fen("rrrrnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_fen_too_many_rows() {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/8 w KQkq - 0 1").unwrap();
    }
}
