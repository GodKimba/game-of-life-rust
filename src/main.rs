use anyhow::{Context, Result};
use game_of_life::Board;
use rand::Rng;
use std::fmt;

fn main() {
    // let board = Board::dead_state(5, 5);
    let board = Board {
        width: 3,
        height: 3,
        board: vec![vec![0, 0, 1], vec![0, 1, 1], vec![0, 0, 0]],
    };
    println!("{:?}", &board);
    let next_state_board = Board::next_board_state(board);
    Board::render(next_state_board);
}
