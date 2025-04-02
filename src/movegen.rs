use std::fmt::Display;

use crate::{
    bitset::Bitset,
    board::Board,
    index_to_pos,
    piece::{Color, Piece},
};

#[derive(Debug)]
pub enum MoveType {
    Normal,
    Capture(Piece),
    Castling,
}

pub struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub piece: Piece,
    pub move_type: MoveType,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Move {
            from,
            to,
            piece,
            move_type,
        } = self;

        let from_pos = index_to_pos((from.0, from.1)).unwrap();
        let to_pos = index_to_pos((to.0, to.1)).unwrap();

        match move_type {
            MoveType::Normal => {
                if *piece == Piece::WhitePawn || *piece == Piece::BlackPawn {
                    write!(f, "{}", to_pos)
                } else {
                    write!(f, "{}{}", piece, to_pos)
                }
            }
            MoveType::Capture(_) => {
                if *piece == Piece::WhitePawn || *piece == Piece::BlackPawn {
                    write!(f, "{}x{}", &from_pos[..1], to_pos)
                } else {
                    write!(f, "{}x{}", piece, to_pos)
                }
            }
            MoveType::Castling => {
                if to.1 == 6 {
                    write!(f, "O-O")
                } else {
                    write!(f, "O-O-O")
                }
            }
        }
    }
}

fn valid_target(board: &Board, moves: &mut Bitset, target_square: u8) -> bool {
    let same_occupied = &board.occupied[board.side_to_move.index()];
    if same_occupied.is_bit_set(target_square) {
        return false;
    }

    moves.set_bit(target_square);

    let opposide_occupied = &board.occupied[board.side_to_move.opposite().index()];
    if opposide_occupied.is_bit_set(target_square) {
        return false;
    }

    true
}

const PAWN_OFFSETS: [(i8, i8); 4] = [(1, -1), (1, 0), (1, 1), (2, 0)];

fn pawn_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let mut moves = Bitset::new(0);

    let (rank, file) = *from;

    let offsets = match board.side_to_move {
        Color::White => if rank == 6 {
            &PAWN_OFFSETS
        } else {
            &PAWN_OFFSETS[..3]
        }
        .iter()
        .map(|(r, f)| (-r, *f))
        .collect(),
        Color::Black => if rank == 1 {
            &PAWN_OFFSETS
        } else {
            &PAWN_OFFSETS[..3]
        }
        .to_vec(),
    };

    for (dr, df) in offsets {
        let (r, f) = (rank as i8 + dr, file as i8 + df);

        if (0..8).contains(&r) && (0..8).contains(&f) {
            let target_square = (r * 8 + f) as u8;
            if df == 0 {
                let occupied = &board.occupied[0];
                if occupied.is_bit_set(target_square) {
                    break;
                }

                moves.set_bit(target_square);
            } else {
                let opposide_occupied = &board.occupied[board.side_to_move.opposite().index()];
                if opposide_occupied.is_bit_set(target_square) {
                    moves.set_bit(target_square);
                }
            }
        }
    }

    moves
}

const KNIGHT_OFFSETS: [(i8, i8); 8] = [
    (1, 2),
    (1, -2),
    (2, 1),
    (2, -1),
    (-1, 2),
    (-1, -2),
    (-2, -1),
    (-2, 1),
];

fn knight_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let mut moves = Bitset::new(0);

    let (rank, file) = *from;

    for (dr, df) in KNIGHT_OFFSETS {
        let (r, f) = (rank as i8 + dr, file as i8 + df);

        if (0..8).contains(&r) && (0..8).contains(&f) {
            let target_square = (r * 8 + f) as u8;
            if !valid_target(board, &mut moves, target_square) {
                continue;
            }
        }
    }

    moves
}

fn sliding_moves(board: &Board, from: &(u8, u8), directions: &[(i8, i8)]) -> Bitset {
    let mut moves = Bitset::new(0);

    let (rank, file) = *from;

    for &(dr, df) in directions {
        let (mut r, mut f) = (rank as i8, file as i8);

        while (0..8).contains(&(r + dr)) && (0..8).contains(&(f + df)) {
            r += dr;
            f += df;

            let target_square = (r * 8 + f) as u8;
            if !valid_target(board, &mut moves, target_square) {
                break;
            }
        }
    }

    moves
}

fn bishop_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    sliding_moves(board, from, &[(1, 1), (1, -1), (-1, 1), (-1, -1)])
}

fn rook_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    sliding_moves(board, from, &[(1, 0), (-1, 0), (0, 1), (0, -1)])
}

fn queen_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    rook_moves(board, from) | bishop_moves(board, from)
}

const KING_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn king_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let mut moves = Bitset::new(0);

    let (rank, file) = *from;

    for (dr, df) in KING_OFFSETS {
        let (r, f) = (rank as i8 + dr, file as i8 + df);

        if (0..8).contains(&r) && (0..8).contains(&f) {
            let target_square = (r * 8 + f) as u8;
            if !valid_target(board, &mut moves, target_square) {
                continue;
            }
        }
    }

    moves
}

fn is_empty(board: &Board, indices: &[(u8, u8)]) -> bool {
    indices
        .iter()
        .map(|(r, f)| !board.occupied[0].is_bit_set(r * 8 + f))
        .reduce(|acc, e| acc && e)
        .unwrap_or(false)
}

fn castling_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let mut moves = Bitset::new(0);

    let king_pos = match board.side_to_move {
        Color::White => (7, 4),
        Color::Black => (0, 4),
    };

    if *from == king_pos {
        let (king_castle_index, queen_castle_index) = (
            2 * board.side_to_move.index() - 2,
            2 * board.side_to_move.index() - 1,
        );

        let king_rook = board.get(king_pos.0, 7);
        if let Some(king_rook) = king_rook {
            if (king_rook == Piece::WhiteRook || king_rook == Piece::BlackRook)
                && king_rook.color() == board.side_to_move
                && board.can_castle[king_castle_index]
                && is_empty(board, &[(king_pos.0, 5), (king_pos.0, 6)])
            {
                let square = king_pos.0 * 8 + 6;
                moves.set_bit(square);
            }
        }

        let queen_rook = board.get(king_pos.0, 0);
        if let Some(queen_rook) = queen_rook {
            if (queen_rook == Piece::WhiteRook || queen_rook == Piece::BlackRook)
                && queen_rook.color() == board.side_to_move
                && board.can_castle[queen_castle_index]
                && is_empty(board, &[(king_pos.0, 1), (king_pos.0, 2), (king_pos.0, 3)])
            {
                let square = king_pos.0 * 8 + 2;
                moves.set_bit(square);
            }
        }
    }

    moves
}

pub fn valid_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let piece = board.get(from.0, from.1);
    if let Some(piece) = piece {
        if piece.color() != board.side_to_move {
            println!("Out of turn move!");
            return Bitset::new(0);
        }

        return match piece {
            Piece::WhitePawn | Piece::BlackPawn => pawn_moves(board, from),
            Piece::WhiteKnight | Piece::BlackKnight => knight_moves(board, from),
            Piece::WhiteBishop | Piece::BlackBishop => bishop_moves(board, from),
            Piece::WhiteRook | Piece::BlackRook => rook_moves(board, from),
            Piece::WhiteQueen | Piece::BlackQueen => queen_moves(board, from),
            Piece::WhiteKing | Piece::BlackKing => {
                king_moves(board, from) | castling_moves(board, from)
            }
        };
    }

    Bitset::new(0)
}

// TODO: Write rest of tests.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bishop_empty_board() {
        let mut board = Board::new();
        board.set(4, 4, Piece::WhiteBishop);

        assert_eq!(
            valid_moves(&board, &(4, 4)),
            Bitset::new(0x8244280028448201)
        );
    }

    #[test]
    fn bishop_same_color_block() {
        let mut board = Board::new();
        board.set(4, 4, Piece::WhiteBishop);

        board.set(3, 3, Piece::WhitePawn);
        board.set(6, 6, Piece::WhitePawn);

        assert_eq!(
            valid_moves(&board, &(4, 4)),
            Bitset::new(0x0204280020408000)
        );
    }

    #[test]
    fn rook_empty_board() {
        let mut board = Board::new();
        board.set(4, 4, Piece::WhiteRook);

        assert_eq!(
            valid_moves(&board, &(4, 4)),
            Bitset::new(0x101010ef10101010)
        );
    }

    #[test]
    fn queen_empty_board() {
        let mut board = Board::new();
        board.set(4, 4, Piece::WhiteQueen);

        assert_eq!(
            valid_moves(&board, &(4, 4)),
            Bitset::new(0x925438ef38549211)
        );
    }
}
