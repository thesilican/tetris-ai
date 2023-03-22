use anyhow::Result;
use common::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;

    let mut bag = Bag::new_rng7(0);
    let mut game = Game::from_bag(&mut bag);
    let mut index = 0;
    let mut child_states = game.child_states(&PERMS_4F);

    println!(
        "{}\n{:?}\n{} of {}",
        child_states[index].game,
        child_states[index].moves().collect::<Vec<_>>(),
        index + 1,
        child_states.len()
    );
    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key? {
            Key::Ctrl('c') => break,
            Key::Left => {
                index = (index + child_states.len() - 1) % child_states.len();
            }
            Key::Right => {
                index = (index + 1) % child_states.len();
            }
            Key::Char(' ') => {
                for game_move in child_states[index].moves() {
                    game.make_move(game_move);
                }
                game.refill_queue(&mut bag);
                child_states = game.child_states(&PERMS_4F);
                index = 0;
                if child_states.is_empty() {
                    println!("No valid child states");
                    break;
                }
            }
            _ => {}
        }
        println!(
            "{}\n{:?}\n{} of {}",
            child_states[index].game,
            child_states[index].moves().collect::<Vec<_>>(),
            index + 1,
            child_states.len()
        );
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
