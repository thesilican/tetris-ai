use processor::load_frames;

fn main() {
    let frames = load_frames();
    dbg!(frames[0].frames.len());
}
