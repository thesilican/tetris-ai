use common::misc::GenericErr;
use processor::{frames_to_replay, load_frames, replay_to_transition_chain};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let frames = load_frames();
    let replay = frames.iter().map(frames_to_replay).next().unwrap();
    let transitions = replay_to_transition_chain(&replay);

    let num_frames = transitions.transitions.len();
    let mut index = 0;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    println!("{}", transitions.name);
    println!("{}", transitions.transitions[0]);

    stdout.activate_raw_mode()?;
    for key in stdin.keys() {
        stdout.suspend_raw_mode()?;
        match key.unwrap() {
            Key::Left => {
                index = (index + num_frames - 1) % num_frames;
            }
            Key::Right => {
                index = (index + 1) % num_frames;
            }
            Key::Ctrl('c') => {
                return Ok(());
            }
            _ => {}
        }
        println!("{}", transitions.transitions[index]);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
