use anyhow::Result;
use rand::Rng;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[derive(Debug, Clone)]
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
                    print!("   ");
                }
                if counter == board.width {
                    println!("");
                    counter = 0;
                }
            }
        }
    }
    pub fn next_board_state(board: Self) -> Self {
        let mut new_board = Board::dead_state(board.height, board.width);
        for i in 1..board.height - 1 {
            for j in 1..board.width - 1 {
                let alive = 1;
                let mut neighboors: Vec<u8> = vec![];

                let mut alive_cells_count = 0;

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

                for cell in &neighboors {
                    if cell == &alive {
                        alive_cells_count += 1
                    }
                }

                match alive_cells_count {
                    x if x == 3 => new_board.board[i][j] = 1,
                    x if x > 3 => new_board.board[i][j] = 0,
                    x if x <= 1 => new_board.board[i][j] = 0,
                    x if x == 2 => new_board.board[i][j] = board.board[i][j],
                    _ => println!("error"),
                }
            }
        }
        new_board
    }
    pub fn infinite_loop(board: Self) {
        sleep(Duration::from_millis(300));
        println!("---------------------------------------------------------");
        Board::render(board.clone());
        Board::infinite_loop(Board::next_board_state(board));
    }
    pub fn search<'a>(contents: &'a str) -> Result<Vec<Vec<u8>>> {
        let mut vec = vec![];
        for line in contents.lines() {
            vec.push(vec![line.parse::<u8>()?]);
        }
        Ok(vec)
    }

    pub fn load_board_state() -> Result<Board> {
        let file = fs::read_to_string("toad.txt")?;
        let mut cnt = 0;
        let vec = Board::search(&file)?;
        // let mut width = 0;
        for _ in file.lines() {
            cnt = cnt + 1;
        }
        Ok(Board {
            width: 6,
            height: cnt,
            board: vec,
        })
    }
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
