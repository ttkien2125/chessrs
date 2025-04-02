use std::fmt::Display;

use crate::{
    bitset::Bitset,
    movegen::{valid_moves, Move, MoveType},
    piece::{Color, Piece},
};

pub const STARTING_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const CASTLING_FLAGS: [char; 4] = ['K', 'Q', 'k', 'q'];

pub struct Board {
    pub pieces: [Bitset; 12],
    pub occupied: [Bitset; 3],
    pub side_to_move: Color,
    pub can_castle: [bool; 4],
}

impl Board {
    pub fn new() -> Self {
        Self {
            pieces: [const { Bitset::new(0) }; 12],
            occupied: [const { Bitset::new(0) }; 3],
            side_to_move: Color::White,
            can_castle: [true; 4],
        }
    }

    pub fn get(&self, rank: u8, file: u8) -> Option<Piece> {
        for (index, bitset) in self.pieces.iter().enumerate() {
            let square = rank * 8 + file;
            if bitset.is_bit_set(square) {
                let piece = Piece::from_index(index);
                return piece;
            }
        }

        None
    }

    pub fn set(&mut self, rank: u8, file: u8, piece: Piece) {
        let square = rank * 8 + file;

        let bitset = &mut self.pieces[piece.index()];
        bitset.set_bit(square);

        self.occupied[0].set_bit(square);

        let occupied = match piece.color() {
            Color::White => &mut self.occupied[1],
            Color::Black => &mut self.occupied[2],
        };

        occupied.set_bit(square);
    }

    pub fn clear(&mut self, rank: u8, file: u8, piece: Piece) {
        let square = rank * 8 + file;

        let bitset = &mut self.pieces[piece.index()];
        bitset.clear_bit(square);

        self.occupied[0].clear_bit(square);

        let occupied = match piece.color() {
            Color::White => &mut self.occupied[1],
            Color::Black => &mut self.occupied[2],
        };

        occupied.clear_bit(square);
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
                        Some(piece) => board.set(rank as u8, file, piece),
                        None => return None,
                    }
                } else if ('1'..='8').contains(&c) {
                    file += c.to_digit(10).unwrap() as u8 - 1;
                } else {
                    return None;
                }

                file += 1;
            }

            if file > 8 || rank >= 8 {
                return None;
            }
        }

        let side_to_move = fen_config[1];
        board.side_to_move = match side_to_move {
            "w" => Color::White,
            "b" => Color::Black,
            _ => panic!("Invalid active color"),
        };

        board.can_castle = [false; 4];
        let can_castle = fen_config[2];
        for flag in can_castle.chars() {
            match flag {
                '-' => break,
                'K' => board.can_castle[0] = true,
                'Q' => board.can_castle[1] = true,
                'k' => board.can_castle[2] = true,
                'q' => board.can_castle[3] = true,
                _ => panic!("Invalid castling flags"),
            }
        }

        Some(board)
    }

    pub fn make_move(&mut self, chess_move: &Move) {
        let Move {
            from,
            to,
            piece,
            move_type,
        } = chess_move;

        let valid_moves = valid_moves(self, from);
        if !valid_moves.is_bit_set(to.0 * 8 + to.1) {
            println!("Not a valid move!");
            return;
        }

        self.clear(from.0, from.1, *piece);
        self.set(to.0, to.1, *piece);

        match move_type {
            MoveType::Normal => {
                if *piece == Piece::WhiteKing {
                    self.can_castle[0] = false;
                    self.can_castle[1] = false;
                } else if *piece == Piece::BlackKing {
                    self.can_castle[2] = false;
                    self.can_castle[3] = false;
                } else if *piece == Piece::WhiteRook {
                    if from.1 == 7 {
                        self.can_castle[0] = false;
                    } else if from.1 == 0 {
                        self.can_castle[1] = false;
                    }
                } else if *piece == Piece::BlackRook {
                    if from.1 == 7 {
                        self.can_castle[2] = false;
                    } else if from.1 == 0 {
                        self.can_castle[3] = false;
                    }
                }
            }

            MoveType::Capture(capture) => {
                self.clear(to.0, to.1, *capture);
            }

            MoveType::Castling => {
                let rook = if self.side_to_move == Color::White {
                    Piece::WhiteRook
                } else {
                    Piece::BlackRook
                };

                if to.1 == 6 {
                    self.clear(to.0, 7, rook);
                    self.set(to.0, 5, rook);
                } else if to.1 == 2 {
                    self.clear(to.0, 0, rook);
                    self.set(to.0, 3, rook);
                } else {
                    panic!()
                }

                let (king_castle_index, queen_castle_index) = (
                    2 * self.side_to_move.index() - 2,
                    2 * self.side_to_move.index() - 1,
                );
                self.can_castle[king_castle_index] = false;
                self.can_castle[queen_castle_index] = false;
            }
        }

        self.side_to_move = self.side_to_move.opposite();
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "-----------------------------------------")?;
        write!(f, "      ")?;
        for file in 'a'..='h' {
            write!(f, "{}   ", file)?;
        }
        writeln!(f)?;

        writeln!(f, "    ---------------------------------    ")?;
        for rank in 0..8 {
            write!(f, "  {} | ", 8 - rank)?;
            for file in 0..8 {
                match self.get(rank, file) {
                    Some(piece) => write!(f, "{} | ", piece),
                    None => write!(f, "- | "),
                }?
            }
            write!(f, "{}", 8 - rank)?;
            writeln!(f)?;
        }
        writeln!(f, "    ---------------------------------    ")?;

        write!(f, "      ")?;
        for file in 'a'..='h' {
            write!(f, "{}   ", file)?;
        }
        writeln!(f)?;
        write!(f, "-----------------------------------------")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_FEN_STRING: &str = "8/8/8/8/8/8/8/8 w KQkq - 0 1";

    #[test]
    fn empty_fen() {
        let board = Board::from_fen(EMPTY_FEN_STRING).unwrap();

        for rank in 0..8 {
            for file in 0..8 {
                assert_eq!(board.get(rank, file), None)
            }
        }
    }

    #[test]
    fn starting_fen() {
        let board = Board::from_fen(STARTING_FEN_STRING).unwrap();

        assert_eq!(board.get(0, 0), Some(Piece::BlackRook));
        assert_eq!(board.get(0, 1), Some(Piece::BlackKnight));
        assert_eq!(board.get(0, 2), Some(Piece::BlackBishop));
        assert_eq!(board.get(0, 3), Some(Piece::BlackQueen));
        assert_eq!(board.get(0, 4), Some(Piece::BlackKing));
        assert_eq!(board.get(0, 5), Some(Piece::BlackBishop));
        assert_eq!(board.get(0, 6), Some(Piece::BlackKnight));
        assert_eq!(board.get(0, 7), Some(Piece::BlackRook));

        for file in 0..8 {
            assert_eq!(board.get(1, file), Some(Piece::BlackPawn));
            assert_eq!(board.get(6, file), Some(Piece::WhitePawn));
        }

        for rank in 2..6 {
            for file in 0..8 {
                assert_eq!(board.get(rank, file), None);
            }
        }

        assert_eq!(board.get(7, 0), Some(Piece::WhiteRook));
        assert_eq!(board.get(7, 1), Some(Piece::WhiteKnight));
        assert_eq!(board.get(7, 2), Some(Piece::WhiteBishop));
        assert_eq!(board.get(7, 3), Some(Piece::WhiteQueen));
        assert_eq!(board.get(7, 4), Some(Piece::WhiteKing));
        assert_eq!(board.get(7, 5), Some(Piece::WhiteBishop));
        assert_eq!(board.get(7, 6), Some(Piece::WhiteKnight));
        assert_eq!(board.get(7, 7), Some(Piece::WhiteRook));
    }

    #[test]
    fn random_fen() {
        let board =
            Board::from_fen("r2q1rk1/2p1bppp/p1n1bn2/1p2p3/4P3/2P2N2/PPBN1PPP/R1BQR1K1 w - - 1 12")
                .unwrap();

        assert_eq!(board.get(0, 0), Some(Piece::BlackRook));
        assert_eq!(board.get(0, 3), Some(Piece::BlackQueen));
        assert_eq!(board.get(0, 6), Some(Piece::BlackKing));

        assert_eq!(board.get(1, 4), Some(Piece::BlackBishop));

        assert_eq!(board.get(2, 2), Some(Piece::BlackKnight));
        assert_eq!(board.get(2, 5), Some(Piece::BlackKnight));

        assert_eq!(board.get(6, 2), Some(Piece::WhiteBishop));

        assert_eq!(board.get(7, 0), Some(Piece::WhiteRook));
        assert_eq!(board.get(7, 3), Some(Piece::WhiteQueen));
        assert_eq!(board.get(7, 6), Some(Piece::WhiteKing));
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
