use anyhow::{Context, Result};
use std::fmt;

fn main() {
    println!("{:?}", Board::dead_state(5, 5));
}

#[derive(Debug)]
struct Board {
    width: Vec<Vec<usize>>,
    heigth: Vec<Vec<usize>>,
}

impl Board {
    fn dead_state(width: usize, heigth: usize) -> Board {
        let width = vec![vec![0; width]; width];
        let heigth = vec![vec![0; heigth]; heigth];
        Board { width, heigth }
    }
}

enum BoardState {
    Alive,
    Dead,
}

fn random_state() {}
