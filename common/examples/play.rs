use anyhow::Result;
use common::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;

    let mut bag = Bag::new_rng7(123456);
    let mut game = Game::from_bag(&mut bag);
    println!("{game}");

    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key? {
            Key::Ctrl('c') => break,
            Key::Left => {
                game.active.shift_left(&game.board);
            }
            Key::Right => {
                game.active.shift_right(&game.board);
            }
            Key::Up => {
                game.active.soft_drop(&game.board);
            }
            Key::Down => {
                let info = game.hard_drop();
                if let Some(LockInfo {
                    lines_cleared,
                    top_out,
                }) = info
                {
                    println!("Drop: Lines cleared: {lines_cleared} Top out: {top_out}");
                }
            }
            Key::Char('a') => {
                game.active.rotate_180(&game.board);
            }
            Key::Char('z') => {
                game.active.rotate_ccw(&game.board);
            }
            Key::Char('x') => {
                game.active.rotate_cw(&game.board);
            }
            Key::Char('c') => {
                game.swap_hold();
            }
            Key::Char('g') => game.board.add_garbage(6, 2),
            _ => {}
        }

        game.refill_queue(&mut bag);
        println!("{game}");
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
