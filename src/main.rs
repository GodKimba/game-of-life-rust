use anyhow::Result;
use game_of_life::Board;

fn main() -> Result<()> {
    // let board = Board::dead_state(20, 30);
    // let board = Board::load_board_state()?;
    let board = Board {
        width: 6,
        height: 6,
        board: vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 1, 0],
            vec![0, 1, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ],
    };
    // Board::render(board.clone());
    // println!("---------------------------------------------");
    // let next = Board::next_board_state(board.clone());
    // Board::render(next.clone());
    // println!("---------------------------------------------");
    // let board = Board::next_board_state(next);
    // Board::render(board);

    loop {
        Board::infinite_loop(board.clone());
    }
}
