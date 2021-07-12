use common::misc::GenericErr;
use common::model::{Bag, ChildStatesOptions, Game, DSDR};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    const CHILD_STATE_MODE: ChildStatesOptions = DSDR;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;

    let mut bag = Bag::new(1);
    let mut game = Game::from_bag(&mut bag, true);
    let mut index = 0;
    let mut child_states = game.child_states(CHILD_STATE_MODE);

    println!(
        "{}\n{:?}\n{} of {}",
        child_states[index].0,
        child_states[index].1,
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
                for game_move in child_states[index].1 {
                    game.make_move(*game_move);
                }
                game.refill_queue(&mut bag, true);
                child_states = game.child_states(CHILD_STATE_MODE);
                index = 0;
                if child_states.len() == 0 {
                    println!("No valid child states");
                    break;
                }
            }
            _ => {}
        }
        println!(
            "{}\n{:?}\n{} of {}",
            child_states[index].0,
            child_states[index].1,
            index + 1,
            child_states.len()
        );
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
