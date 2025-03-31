#![allow(dead_code)]

mod bitset;
mod board;
mod movegen;
mod piece;

use board::{Board, STARTING_FEN_STRING};
use movegen::valid_moves;
use piece::{Color, Piece};

struct Move {
    pub from: (u8, u8),
    pub to: (u8, u8),
    pub piece: Piece,
    pub capture: Option<Piece>,
}

struct Game {
    pub board: Board,
}

impl Game {
    pub fn new() -> Self {
        // let board = Board::from_fen(STARTING_FEN_STRING).unwrap();

        let mut board = Board::new();

        board.set(3, 1, Piece::WhiteRook);
        board.set(1, 1, Piece::BlackQueen);
        board.set(4, 1, Piece::WhitePawn);
        board.set(4, 6, Piece::BlackPawn);
        board.set(2, 4, Piece::WhiteKnight);
        board.set(6, 4, Piece::BlackKnight);

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

fn main() {
    let mut game = Game::new();

    loop {
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

        'inner: loop {
            println!("Make your move:");

            let mut chess_move = String::new();

            std::io::stdin().read_line(&mut chess_move).unwrap();

            let chess_move = chess_move.trim();
            if chess_move == "q" {
                return;
            }

            if chess_move.len() == 2 {
                let mut moves = Vec::new();
                let from = pos_to_index(chess_move);

                if let Some(from) = from {
                    let valid_moves = valid_moves(&game.board, &from);

                    for rank in 0..8 {
                        for file in 0..8 {
                            let square = rank * 8 + file;
                            if valid_moves.is_bit_set(square) {
                                moves.push(index_to_pos((rank, file)).unwrap());
                            }
                        }
                    }

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
                    if let Some(piece) = game.board.get(from.0, from.1) {
                        let capture = game.board.get(to.0, to.1);

                        let chess_move = Move {
                            from,
                            to,
                            piece,
                            capture,
                        };
                        game.board.make_move(&chess_move);
                    } else {
                        println!("Invalid piece at starting square!");
                    }
                } else {
                    println!("Out of bounds position!");
                }
            } else {
                println!("Wrong move syntax!");
            }

            break 'inner;
        }
    }
}
