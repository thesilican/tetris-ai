use common::misc::GenericErr;
use processor::{FrameCollection, Replay};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let frames = FrameCollection::load();
    let mut replay = frames
        .iter()
        .map(|f| Replay::from_frame_collection(f))
        .next()
        .unwrap();
    let keyframes = replay.keyframes().clone();
    let num_frames = keyframes.len();
    let mut index = 0;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    println!();
    println!("{}", replay.name);
    println!("{}", keyframes[0]);

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
        println!("{}", keyframes[index]);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
