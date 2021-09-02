use processor::{frame_collection_to_replay, load_frame_collections};

fn main() {
    let frames = load_frame_collections();
    let replay = frame_collection_to_replay(&frames[0]);
}
