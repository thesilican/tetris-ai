use c4w_ai::model::consts::BOARD_HEIGHT;
use c4w_ai::model::consts::BOARD_WIDTH;
use c4w_ai::model::consts::PIECE_SHAPE_SIZE;
use c4w_ai::model::game::Game;
use c4w_ai::model::game::GameMove;
use c4w_ai::model::piece::Piece;
use c4w_ai::model::piece::PieceType;
use std::fmt::Write;
use std::io::{stdin, stdout, Write as IOWrite};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn write_state(stdout: &mut termion::raw::RawTerminal<std::io::Stdout>, game: &Game) {
    let mut text = String::new();
    let piece = &game.current_piece;
    let board = &game.board;

    // Print board/shape combo
    let piece_shape = piece.get_shape(None);
    let (p_x, p_y) = piece.location;
    for j in (0..BOARD_HEIGHT).rev() {
        for i in 0..BOARD_WIDTH {
            let in_piece_bounds = i - p_x >= 0
                && i - p_x < PIECE_SHAPE_SIZE
                && j - p_y >= 0
                && j - p_y < PIECE_SHAPE_SIZE;
            let in_piece = in_piece_bounds && piece_shape[(i - p_x) as usize][(j - p_y) as usize];

            if in_piece {
                write!(text, "██").unwrap();
            } else if board.get(i, j) {
                write!(text, "▓▓").unwrap();
            } else if in_piece_bounds {
                write!(text, "▒▒").unwrap();
            } else {
                write!(text, "░░").unwrap();
            }
        }
        writeln!(text).unwrap();
    }
    // Board height/holes info
    for i in 0..BOARD_WIDTH {
        let height = board.height_map[i as usize];
        write!(text, "{:2}", height).unwrap();
    }
    writeln!(text).unwrap();
    for i in 0..BOARD_WIDTH {
        let hole = board.holes[i as usize];
        write!(text, "{:2}", hole).unwrap();
    }
    writeln!(text).unwrap();
    // Other info
    let curr = &game.current_piece.to_string();
    let hold = match &game.hold_piece {
        Some(piece) => piece.to_string(),
        None => String::from("null"),
    };
    let mut queue_text = String::new();
    for piece in &game.queue_pieces {
        queue_text.push_str(&piece.to_string());
        queue_text.push(' ');
    }
    writeln!(text, "Curr: {} Hold: {} Queue: {}", curr, hold, queue_text).unwrap();
    writeln!(
        text,
        "Can hold: {} Was hold empty: {}",
        game.can_hold, game.hold_was_empty
    )
    .unwrap();
    // Print
    writeln!(stdout, "{}", text).unwrap();
    stdout.flush().unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.suspend_raw_mode().unwrap();

    let mut game = Game::new();
    let mut undo_stack = Vec::new();
    game.set_current(Piece::new(PieceType::O));
    game.queue_pieces.extend(vec![
        Piece::new(PieceType::I),
        Piece::new(PieceType::T),
        Piece::new(PieceType::L),
        Piece::new(PieceType::J),
        Piece::new(PieceType::S),
        Piece::new(PieceType::Z),
    ]);

    write_state(&mut stdout, &game);
    stdout.activate_raw_mode().unwrap();
    for c in stdin.keys() {
        stdout.suspend_raw_mode().unwrap();
        match c.unwrap() {
            Key::Ctrl('c') => return,
            Key::Left => {
                game.make_move(&GameMove::ShiftLeft).ok();
            }
            Key::Right => {
                game.make_move(&GameMove::ShiftRight).ok();
            }
            Key::Up => {
                let res = game.make_move(&GameMove::HardDrop).ok();
                if let Some(Some(res)) = res {
                    let (_res, undo) = res;
                    undo_stack.push(undo);
                    if game.queue_pieces.len() < 7 {
                        game.queue_pieces.extend(vec![
                            Piece::new(PieceType::O),
                            Piece::new(PieceType::I),
                            Piece::new(PieceType::T),
                            Piece::new(PieceType::L),
                            Piece::new(PieceType::J),
                            Piece::new(PieceType::S),
                            Piece::new(PieceType::Z),
                        ]);
                    }
                }
            }
            Key::Down => {
                game.make_move(&GameMove::SoftDrop).ok();
            }
            Key::Char('z') => {
                game.make_move(&GameMove::RotateLeft).ok();
            }
            Key::Char('x') => {
                game.make_move(&GameMove::RotateRight).ok();
            }
            Key::Char('a') => {
                game.make_move(&GameMove::Rotate180).ok();
            }
            Key::Char('c') => {
                game.make_move(&GameMove::Hold).ok();
            }
            Key::Char('u') => {
                if let Some(undo) = undo_stack.pop() {
                    game.undo_move(undo);
                }
            }
            _ => {}
        }

        write_state(&mut stdout, &game);
        stdout.activate_raw_mode().unwrap();
    }
}
