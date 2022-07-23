use anyhow::{Context, Result};
use rand::Rng;
use std::mem;

#[derive(Debug)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    pub board: Vec<Vec<u8>>,
}

impl Board {
    pub fn dead_state(height: usize, width: usize) -> Self {
        let board = vec![vec![0; width]; height];
        Board {
            height,
            width,
            board,
        }
    }
    pub fn random_state(mut board: Self) -> Self {
        for i in 0..board.height {
            for j in 0..board.width {
                let random = rand::thread_rng().gen_range(-1, 1);
                board.board[i][j] = if random == 0 { 1 } else { 0 }
            }
        }
        board
    }
    pub fn render(board: Self) {
        for i in 0..board.height {
            let mut counter = 0;
            for j in 0..board.width {
                counter += 1;
                if board.board[i][j] == 1 {
                    print!(" # ");
                } else {
                    print!(" - ");
                }
                if counter == board.width {
                    println!("");
                    counter = 0;
                }
            }
        }
    }
    pub fn next_board_state(boardd: Self) -> Self {
        let mut total_turns_count = 0;
        let mut board = boardd;
        while total_turns_count < board.height {
            for i in 0..board.height - 1 {
                for j in 0..board.width - 1 {
                    let mut neighboors: Vec<u8> = vec![];
                    if i > 0 {
                        let above_cell = board.board[i - 1][j];
                        neighboors.push(above_cell);
                    }

                    if i < board.height {
                        let below_cell = board.board[i + 1][j];
                        neighboors.push(below_cell);
                    }

                    if j > 0 {
                        let left_cell = board.board[i][j - 1];
                        neighboors.push(left_cell);
                    }

                    if j < board.width {
                        let right_cell = board.board[i][j + 1];
                        neighboors.push(right_cell);
                    }

                    if j > 0 && i > 0 {
                        let above_left_cell = board.board[i - 1][j - 1];
                        neighboors.push(above_left_cell);
                    }

                    if j < board.width && i < board.height {
                        let below_right_cell = board.board[i + 1][j + 1];
                        neighboors.push(below_right_cell);
                    }

                    if j > 0 && i < board.height {
                        let below_left_cell = board.board[i + 1][j - 1];
                        neighboors.push(below_left_cell);
                    }

                    if j < board.width && i > 0 {
                        let above_right_cell = board.board[i - 1][j + 1];
                        neighboors.push(above_right_cell);
                    }

                    let current_cell = board.board[i][j];
                    let mut alive_cells_count = 0;
                    if current_cell == 1 {
                        alive_cells_count += 1
                    }
                    for cell in neighboors {
                        if cell == 1 {
                            alive_cells_count += 1
                        }
                    }
                    if alive_cells_count == 3 {
                        board.board[i][j] = 1;
                    } else if alive_cells_count <= 1 {
                        board.board[i][j] = 0;
                    } else if alive_cells_count == 2 {
                        board.board[i][j] = board.board[i][j];
                        continue;
                    } else if alive_cells_count > 3 {
                        board.board[i][j] = 0;
                    }
                }
                // needs to be here for while loop to work
                total_turns_count += 1;
            }
        }
        board
    }
}

enum BoardState {
    Alive,
    Dead,
}

mod tests {
    use crate::Board;
    #[test]
    fn change_state_test() {
        let board = Board {
            width: 3,
            height: 3,
            board: vec![vec![0, 0, 1], vec![0, 1, 1], vec![0, 0, 0]],
        };
        let next_state = Board::next_board_state(board);
        let expected_state = Board {
            width: 3,
            height: 3,
            board: vec![vec![0, 1, 1], vec![0, 1, 1], vec![0, 0, 0]],
        };
        assert_eq!(next_state.board, expected_state.board);
    }

    #[test]
    fn keep_state() {
        let board = Board {
            width: 3,
            height: 3,
            board: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        };
        let next_state = Board::next_board_state(board);
        let expected_state = Board {
            width: 3,
            height: 3,
            board: vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]],
        };
        assert_eq!(next_state.board, expected_state.board);
    }
}
