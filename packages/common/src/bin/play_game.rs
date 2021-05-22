use common::misc::GenericErr;
use common::model::game::Game;
use common::model::game::GameMove;
use common::model::game::GameMoveRes;
use common::model::piece::PieceType;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;

    let mut game = Game::new();
    game.extend_queue(&PieceType::iter_types().skip(1).collect::<Vec<_>>());
    println!("{}", std::mem::size_of::<Game>());
    println!("{}", game);

    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key? {
            Key::Ctrl('c') => break,
            Key::Left => {
                game.make_move(&GameMove::ShiftLeft);
            }
            Key::Right => {
                game.make_move(&GameMove::ShiftRight);
            }
            Key::Up => {
                game.make_move(&GameMove::SoftDrop);
            }
            Key::Down => {
                if let GameMoveRes::SuccessDrop(drop_info) = game.make_move(&GameMove::HardDrop) {
                    println!(
                        "Drop: Lines cleared: {} Top out: {}",
                        drop_info.lines_cleared, drop_info.top_out
                    );
                }
            }
            Key::Char('a') => {
                game.make_move(&GameMove::Rotate180);
            }
            Key::Char('z') => {
                game.make_move(&GameMove::RotateLeft);
            }
            Key::Char('x') => {
                game.make_move(&GameMove::RotateRight);
            }
            Key::Char('c') => {
                game.make_move(&GameMove::Hold);
            }
            _ => {}
        }

        if game.queue_pieces.len() < 7 {
            refill_queue(&mut game);
        }
        println!("{}", game);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}

fn refill_queue(game: &mut Game) {
    game.extend_queue(&PieceType::iter_types().collect::<Vec<_>>());
}
