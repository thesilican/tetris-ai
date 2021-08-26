use processor::{frames_to_replay, load_frames};

fn main() {
    let frames = load_frames();
    let replay = frames_to_replay(&frames[0]);
}
