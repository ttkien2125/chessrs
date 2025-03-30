use std::fmt::Display;

use crate::{
    bitset::Bitset,
    piece::{Color, Piece},
    Move,
};

pub const STARTING_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub struct Board {
    pub pieces: [Bitset; 12],
    // pub occupancy: [Bitset; 3],
    pub side_to_move: Color,
}

impl Board {
    pub fn new() -> Self {
        let pieces = [const { Bitset::new(0) }; 12];
        // let occupancy = [const { Bitset(0) }; 3];

        Self {
            pieces,
            side_to_move: Color::White,
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
        let bitset = &mut self.pieces[piece.index()];
        let square = rank * 8 + file;

        bitset.set_bit(square);
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

        Some(board)
    }

    pub fn is_move_valid(&self, chess_move: &Move) -> bool {
        let Move {
            from, to, piece, ..
        } = chess_move;

        if self.side_to_move != piece.color() {
            println!("Out of turn move!");
            return false;
        }

        let from_square = from.0 * 8 + from.1;
        let _to_square = to.0 * 8 + to.1;

        let bitset = &self.pieces[piece.index()];
        if !bitset.is_bit_set(from_square) {
            println!("Invalid piece at starting square!");
            return false;
        }

        // let legal_moves = match piece {
        //     _ => unimplemented!("{}", piece),
        // };
        //
        // if !legal_moves.is_bit_set(to_square) {
        //     return false;
        // }

        // TODO: See if king is in check.

        true
    }

    pub fn make_move(&mut self, chess_move: &Move) {
        let Move {
            from,
            to,
            piece,
            capture,
        } = chess_move;

        if !self.is_move_valid(chess_move) {
            return;
        }

        let from_square = from.0 * 8 + from.1;
        let to_square = to.0 * 8 + to.1;

        let bitset = &mut self.pieces[piece.index()];
        bitset.clear_bit(from_square);
        bitset.set_bit(to_square);

        if let Some(capture) = capture {
            let bitset = &mut self.pieces[capture.index()];
            bitset.clear_bit(to_square);
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
