use crate::{bitset::Bitset, board::Board, piece::Piece};

fn sliding_moves(board: &Board, from: &(u8, u8), directions: &[(i8, i8)]) -> Bitset {
    let mut moves = Bitset::new(0);

    let (rank, file) = *from;

    for &(dr, df) in directions {
        let (mut r, mut f) = (rank as i8, file as i8);

        while (0..8).contains(&(f + df)) && (0..8).contains(&(r + dr)) {
            r += dr;
            f += df;

            let target_square = (r * 8 + f) as u8;

            let same_occupied = &board.occupied[board.side_to_move.index()];
            if same_occupied.is_bit_set(target_square) {
                break;
            }

            moves.set_bit(target_square);

            let opposide_occupied = &board.occupied[board.side_to_move.opposite().index()];
            if opposide_occupied.is_bit_set(target_square) {
                break;
            }
        }
    }

    moves
}

fn rook_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    sliding_moves(board, from, &[(1, 0), (-1, 0), (0, 1), (0, -1)])
}

fn bishop_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    sliding_moves(board, from, &[(1, 1), (1, -1), (-1, 1), (-1, -1)])
}

fn queen_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    rook_moves(board, from) | bishop_moves(board, from)
}

pub fn valid_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    let piece = board.get(from.0, from.1);
    if let Some(piece) = piece {
        if piece.color() != board.side_to_move {
            println!("Out of turn move!");
            return Bitset::new(0);
        }

        // let bitset = &board.pieces[piece.index()];
        // let from_square = from.0 * 8 + from.1;
        // if !bitset.is_bit_set(from_square) {
        //     println!("Invalid piece at starting square!");
        //     return Bitset::new(0);
        // }

        return match piece {
            Piece::WhitePawn | Piece::BlackPawn => todo!(),
            Piece::WhiteKnight | Piece::BlackKnight => todo!(),
            Piece::WhiteBishop | Piece::BlackBishop => bishop_moves(board, from),
            Piece::WhiteRook | Piece::BlackRook => rook_moves(board, from),
            Piece::WhiteQueen | Piece::BlackQueen => queen_moves(board, from),
            Piece::WhiteKing | Piece::BlackKing => todo!(),
        };
    }

    Bitset::new(0)
}
