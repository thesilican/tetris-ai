use anyhow::Result;
use common::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;

    let mut bag = Bag::new(123456);
    let mut game = Game::from_bag_shuffled(&mut bag);
    println!("{}", game);

    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key? {
            Key::Ctrl('c') => break,
            Key::Left => {
                game.make_move(GameMove::ShiftLeft);
            }
            Key::Right => {
                game.make_move(GameMove::ShiftRight);
            }
            Key::Up => {
                game.make_move(GameMove::SoftDrop);
            }
            Key::Down => {
                let res = game.make_move(GameMove::HardDrop);
                if let GameActionRes::SuccessDrop {
                    lines_cleared,
                    top_out,
                } = res
                {
                    println!(
                        "Drop: Lines cleared: {} Top out: {}",
                        lines_cleared, top_out
                    );
                }
            }
            Key::Char('a') => {
                game.make_move(GameMove::Rotate180);
            }
            Key::Char('z') => {
                game.make_move(GameMove::RotateCCW);
            }
            Key::Char('x') => {
                game.make_move(GameMove::RotateCW);
            }
            Key::Char('c') => {
                game.make_move(GameMove::Hold);
            }
            Key::Char('g') => game.board.add_garbage(6, 2),
            _ => {}
        }

        game.refill_queue_shuffled(&mut bag);
        println!("{}", game);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
