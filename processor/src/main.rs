use processor::{FrameCollection, Replay};

fn main() {
    let frames = FrameCollection::load();
    let replays = frames
        .iter()
        .map(Replay::from_frame_collection)
        .collect::<Vec<_>>();
    println!("Finished computing replays");
}
