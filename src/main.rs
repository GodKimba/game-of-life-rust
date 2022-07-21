use anyhow::{Context, Result};
use rand::Rng;
use std::fmt;

fn main() {
    let board = Board::dead_state(5, 5);
    let board_random = Board::random_state(board);
    println!("{:?}", &board_random);
    Board::render(board_random);
}

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<u8>>,
}

impl Board {
    fn dead_state(height: usize, width: usize) -> Self {
        let board = vec![vec![0; width]; height];
        Board {
            height,
            width,
            board,
        }
    }
    fn random_state(mut board: Self) -> Self {
        for i in 0..board.height {
            for j in 0..board.width {
                let random = rand::thread_rng().gen_range(-1, 1);
                board.board[i][j] = if random == 0 { 1 } else { 0 }
            }
        }
        board
    }
    fn render(board: Self) {
        for i in 0..board.height {
            let mut counter = 0;
            for j in 0..board.width {
                counter += 1;
                if board.board[i][j] == 1 {
                    print!(" # ");
                } else {
                    print!(" - ");
                }
                if counter == board.height {
                    println!("");
                    counter = 0;
                }
            }
        }
    }
}

enum BoardState {
    Alive,
    Dead,
}
