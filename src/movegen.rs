use crate::{bitset::Bitset, board::Board};

fn sliding_moves(board: &Board, from: &(u8, u8), directions: &[(i8, i8)]) -> Bitset {
    let mut moves = Bitset::new(0);

    let (file, rank) = *from;

    for &(df, dr) in directions {
        let (mut f, mut r) = (file as i8, rank as i8);

        while (0..8).contains(&(f + df)) && (0..8).contains(&(r + dr)) {
            f += df;
            r += dr;

            let target_square = r * 8 + f;
            moves.set_bit(target_square as u8);
        }
    }

    moves
}

fn rook_moves(board: &Board, from: &(u8, u8)) -> Bitset {
    sliding_moves(board, from, &[(1, 0), (-1, 0), (0, 1), (0, -1)])
}
