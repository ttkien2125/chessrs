#![allow(dead_code)]

mod bitset;
mod board;
mod movegen;
mod piece;

use bitset::Bitset;
use board::{Board, CASTLING_FLAGS, STARTING_FEN_STRING};
use movegen::{valid_moves, Move, MoveType};
use piece::{Color, Piece};

struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        let board = Board::from_fen(
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R3KB1R w KQkq - 1 1",
        )
        // Board::from_fen(STARTING_FEN_STRING)
        .unwrap();

        // let mut board = Board::new();
        //
        // board.set(3, 1, Piece::WhiteRook);
        // board.set(1, 1, Piece::BlackQueen);
        // board.set(4, 1, Piece::WhitePawn);
        // board.set(4, 6, Piece::BlackPawn);
        // board.set(2, 4, Piece::WhiteKnight);
        // board.set(6, 4, Piece::BlackKnight);

        Self { board }
    }
}

fn pos_to_index(pos: &str) -> Option<(u8, u8)> {
    if pos.len() == 2 {
        let mut pos = pos.chars();

        let file = pos.next().unwrap();
        let rank = pos.next().unwrap().to_digit(10).unwrap() as u8;

        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some((8 - rank, file as u8 - b'a'))
        } else {
            None
        }
    } else {
        panic!("Invalid position!")
    }
}

fn index_to_pos(index: (u8, u8)) -> Option<String> {
    if (0..8).contains(&index.0) && (0..8).contains(&index.1) {
        let rank = 8 - index.0;
        let file = b'a' + index.1;

        return Some(format!("{}{}", file as char, rank));
    }
    None
}

fn bitset_to_pos(bitset: Bitset) -> Vec<String> {
    let mut result = Vec::new();
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            if bitset.is_bit_set(square) {
                result.push(index_to_pos((rank, file)).unwrap());
            }
        }
    }
    result
}

const CASTLING_MOVES: [((u8, u8), (u8, u8)); 4] = [
    ((7, 4), (7, 6)),
    ((7, 4), (7, 2)),
    ((0, 4), (0, 6)),
    ((0, 4), (0, 2)),
];

fn parse_move(board: &Board, from: (u8, u8), to: (u8, u8)) -> Move {
    if let Some(piece) = board.get(from.0, from.1) {
        let move_type = if CASTLING_MOVES.contains(&(from, to)) {
            MoveType::Castling
        } else if let Some(capture) = board.get(to.0, to.1) {
            MoveType::Capture(capture)
        } else {
            MoveType::Normal
        };

        Move {
            from,
            to,
            piece,
            move_type,
        }
    } else {
        panic!("Invalid piece at starting square!");
    }
}

fn current_side_moves(board: &Board) -> Vec<Move> {
    let mut current_moves = Vec::new();

    for square in 0..64 {
        let occupied = &board.occupied[board.side_to_move.index()];
        if occupied.is_bit_set(square) {
            let from = (square / 8, square % 8);
            let valid = valid_moves(board, &from);

            for rank in 0..8 {
                for file in 0..8 {
                    let to = (rank, file);
                    if valid.is_bit_set(to.0 * 8 + to.1) {
                        let chess_move = parse_move(board, from, to);
                        current_moves.push(chess_move);
                    }
                }
            }
        }
    }

    current_moves
}

fn main() {
    let mut game = Game::new();

    loop {
        println!("Turn: {}", game.board.side_to_move);

        print!("Castling: ");
        for (index, flag) in game.board.can_castle.iter().enumerate() {
            if *flag {
                print!("{}", CASTLING_FLAGS[index]);
            } else {
                print!("-");
            }
        }
        println!();

        println!("{}", game.board);

        println!("Bitsets:");
        for (index, bitset) in game.board.pieces.iter().enumerate() {
            println!("    {}     - {}", Piece::from_index(index).unwrap(), bitset);
        }

        println!("Occupied:");
        for (index, bitset) in game.board.occupied.iter().enumerate() {
            let color = if index > 0 {
                format!("{}", Color::from_index(index).unwrap())
            } else {
                "Both ".to_string()
            };

            println!("    {} - {}", color, bitset);
        }

        const MAX_MOVES: usize = 50;
        let current_side_moves = current_side_moves(&game.board);
        let current_side_moves: Vec<_> = current_side_moves
            .iter()
            .map(|m| m.to_string())
            .take(MAX_MOVES)
            .collect();

        println!(
            "{} moves ({} shown): {}",
            game.board.side_to_move,
            current_side_moves.len(),
            current_side_moves.join(", "),
        );

        'inner: loop {
            println!("Make your move:");

            let mut chess_move = String::new();

            std::io::stdin().read_line(&mut chess_move).unwrap();

            let chess_move = chess_move.trim();
            if chess_move == "q" {
                return;
            }

            if chess_move.len() == 2 {
                let from = pos_to_index(chess_move);

                if let Some(from) = from {
                    let valid_moves = valid_moves(&game.board, &from);
                    let moves = bitset_to_pos(valid_moves);

                    let from_pos = index_to_pos(from).unwrap();
                    println!("Valid moves from {}: {}", from_pos, moves.join(", "));
                }

                continue 'inner;
            } else if chess_move.len() == 4 {
                let start_square = &chess_move[0..2];
                let end_square = &chess_move[2..];

                let from = pos_to_index(start_square);
                let to = pos_to_index(end_square);

                if let (Some(from), Some(to)) = (from, to) {
                    let chess_move = parse_move(&game.board, from, to);
                    game.board.make_move(&chess_move);
                } else {
                    panic!("Out of bounds position!");
                }
            } else {
                println!("Wrong move syntax!");
            }

            break 'inner;
        }
    }
}
