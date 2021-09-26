use common::misc::GenericErr;
use processor::{FrameCollection, Replay};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let frames = FrameCollection::load();
    let replays = frames
        .iter()
        .take(1)
        .map(|f| Replay::from_frame_collection(f))
        .collect::<Vec<_>>();
    let replay = replays[0].clone();
    let frames = replay.frames().clone();
    let num_frames = frames.len();
    let mut index = 0;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    println!();
    println!("{}", replay.name);
    println!("{}", frames[0]);

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
        println!("{}", frames[index]);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
