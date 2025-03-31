use crate::{bitset::Bitset, board::Board, piece::Piece};

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
        let (mut r, mut f) = (rank as i8, file as i8);

        if (0..8).contains(&(r + dr)) && (0..8).contains(&(f + df)) {
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
        let (mut r, mut f) = (rank as i8, file as i8);

        if (0..8).contains(&(r + dr)) && (0..8).contains(&(f + df)) {
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

pub fn valid_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let piece = board.get(from.0, from.1);
    if let Some(piece) = piece {
        if piece.color() != board.side_to_move {
            println!("Out of turn move!");
            return Bitset::new(0);
        }

        return match piece {
            Piece::WhitePawn | Piece::BlackPawn => todo!(),
            Piece::WhiteKnight | Piece::BlackKnight => knight_moves(board, from),
            Piece::WhiteBishop | Piece::BlackBishop => bishop_moves(board, from),
            Piece::WhiteRook | Piece::BlackRook => rook_moves(board, from),
            Piece::WhiteQueen | Piece::BlackQueen => queen_moves(board, from),
            Piece::WhiteKing | Piece::BlackKing => king_moves(board, from),
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
