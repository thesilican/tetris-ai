use common::misc::GenericErr;
use processor::load_frame_collections;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() -> Result<(), GenericErr> {
    let frames = load_frame_collections();
    let frame_collection = frames[0].clone();
    let num_frames = frame_collection.frames.len();
    let mut index = 0;

    let stdin = std::io::stdin();
    let stdout = std::io::stdout().into_raw_mode()?;
    stdout.suspend_raw_mode()?;
    println!("{}", frame_collection.name);
    println!("{}", frame_collection.frames[index]);

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
        println!("{}", frame_collection.frames[index]);
        stdout.activate_raw_mode()?;
    }
    Ok(())
}
