use c4w_ai::model::board::Board;
use c4w_ai::model::consts::BOARD_HEIGHT;
use c4w_ai::model::consts::BOARD_WIDTH;
use std::fmt::Write;

fn print_board(board: &Board) {
    let mut text = String::new();
    for j in (0..BOARD_HEIGHT).rev() {
        for i in 0..BOARD_WIDTH {
            if board.get(i, j) {
                write!(text, "██").unwrap();
            } else {
                write!(text, "░░").unwrap();
            }
        }
        writeln!(text).unwrap();
    }
    for i in 0..BOARD_WIDTH {
        let height = board.height_map[i as usize];
        write!(text, "{:2}", height).unwrap();
    }
    writeln!(text).unwrap();
    for i in 0..BOARD_WIDTH {
        let hole = board.holes[i as usize];
        write!(text, "{:2}", hole).unwrap();
    }
    println!("{}", text);
}

fn main() {
    let mut board = Board::new();
    let mut buffer = String::new();
    loop {
        print_board(&board);
        buffer.clear();
        std::io::stdin().read_line(&mut buffer).unwrap();
        let mut splits = buffer.split(' ');
        let x = splits.next().unwrap().trim().parse::<i32>().unwrap();
        let y = splits.next().unwrap().trim().parse::<i32>().unwrap();
        board.set(x, y, !board.get(x, y));
    }
}
