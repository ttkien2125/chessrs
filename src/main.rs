#![allow(dead_code)]

mod core;
use core::{board::*, piece::*};

struct Move {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Self {
        let mut board = Board::new();
        // let mut board = Board::starting_pos();

        board[(4, 4)] = Square(Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }));

        board[(4, 1)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));

        board[(4, 6)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));

        board[(2, 4)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }));

        board[(6, 4)] = Square(Some(Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }));

        Self {
            board,
            turn: Color::White,
        }
    }
}

fn pos_to_index(pos: &str) -> Option<(usize, usize)> {
    if pos.len() == 2 {
        let mut pos = pos.chars();
        let file = pos.next().unwrap();
        let rank = pos.next().unwrap().to_digit(10).unwrap() as usize;

        if ('a'..='h').contains(&file) && (1..=8).contains(&rank) {
            Some((8 - rank, file as usize - 'a' as usize))
        } else {
            None
        }
    } else {
        panic!("Invalid position!")
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        game.board.print();

        println!("Make your move:");

        let mut chess_move = String::new();

        std::io::stdin()
            .read_line(&mut chess_move)
            .expect("Failed to read line");

        let chess_move = chess_move.trim();
        if chess_move.len() == 4 {
            let start_square = &chess_move[0..2];
            let end_square = &chess_move[2..];

            let start = pos_to_index(start_square);
            let end = pos_to_index(end_square);

            if let (Some(start), Some(end)) = (start, end) {
                let chess_move = Move { start, end };
                game.board.make_move(&chess_move);
            } else {
                println!("Out of bounds position!");
                continue;
            }
        } else {
            println!("Wrong move syntax!");
            continue;
        }
    }
}
