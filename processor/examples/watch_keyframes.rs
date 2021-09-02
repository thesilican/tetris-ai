use common::misc::GenericErr;
use processor::{frame_collection_to_replay, load_frame_collections};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let frames = load_frame_collections();
    let replay = frames
        .iter()
        .map(frame_collection_to_replay)
        .next()
        .unwrap();

    let num_frames = replay.keyframes.len();
    let mut index = 0;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    println!();
    println!("{}", replay.name);
    println!("{}", replay.keyframes[0]);

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
        println!("{}", replay.keyframes[index]);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
