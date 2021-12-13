use common::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;

    let mut bag = Bag::new(0);
    let mut game = Game::from_bag_shuffled(&mut bag);
    let mut index = 0;
    let mut child_states = game.child_states(&MOVES_3F);

    println!(
        "{}\n{:?}\n{} of {}",
        child_states[index].game,
        child_states[index].moves,
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
                for game_move in child_states[index].moves {
                    game.make_move(*game_move);
                }
                game.refill_queue_shuffled(&mut bag);
                child_states = game.child_states(&MOVES_3F);
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
            child_states[index].game,
            child_states[index].moves,
            index + 1,
            child_states.len()
        );
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
