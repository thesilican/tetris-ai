use anyhow::Result;
use libtetris::*;
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
    // let mut children = game.children_fast();
    let mut children = game.children(4);

    println!(
        "{}\n{:?}\n{} of {}",
        children[index].game,
        children[index].actions().collect::<Vec<_>>(),
        index + 1,
        children.len()
    );
    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key? {
            Key::Ctrl('c') => break,
            Key::Left => {
                index = (index + children.len() - 1) % children.len();
            }
            Key::Right => {
                index = (index + 1) % children.len();
            }
            Key::Char(' ') => {
                for action in children[index].actions() {
                    game.apply(action);
                }
                game.refill_queue(&mut bag);
                children = game.children(4);
                index = 0;
                if children.is_empty() {
                    println!("No valid child states");
                    break;
                }
            }
            _ => {}
        }
        println!(
            "{}\n{:?}\n{} of {}",
            children[index].game,
            children[index].actions().collect::<Vec<_>>(),
            index + 1,
            children.len()
        );
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
