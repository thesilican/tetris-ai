use common::model::*;
use std::{ffi::OsStr, fs};

#[derive(Debug, Clone)]
pub struct FrameCollection {
    pub name: String,
    pub frames: Vec<Game>,
}

// Some frames are when the piece spawns then immediately shifts one space down
// Remove the first of these pair of frames
pub fn remove_shift_down_frames(frame_collection: &mut FrameCollection) {
    let mut to_remove = Vec::new();
    for (i, frame) in frame_collection.frames.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let mut prev_frame = frame_collection.frames[i - 1];
        if prev_frame.current_piece.location != *prev_frame.current_piece.get_spawn_location() {
            continue;
        }
        prev_frame.current_piece.shift_down(&prev_frame.board);
        if prev_frame != *frame {
            continue;
        }
        to_remove.push(i - 1);
    }
    for i in to_remove.into_iter().rev() {
        frame_collection.frames.remove(i);
    }
}

pub fn load_frames() -> Vec<FrameCollection> {
    println!("Loading frames...");
    let paths = fs::read_dir("data/frames").unwrap();
    let mut frame_collections = Vec::new();
    for path in paths {
        let path = path.unwrap();
        let file_name = path.path();
        // println!("{:?} {:?}", file_name, file_name.extension());
        if file_name.extension() != Some(OsStr::new("json")) {
            continue;
        }
        let text = fs::read_to_string(file_name).unwrap();
        let frames = serde_json::from_str(&text).unwrap();
        let mut frame_collection = FrameCollection {
            name: path.path().file_stem().unwrap().to_str().unwrap().into(),
            frames,
        };
        remove_shift_down_frames(&mut frame_collection);
        frame_collections.push(frame_collection);
    }
    // Sort by name alphabetically
    frame_collections.sort_by(|a, b| a.name.cmp(&b.name));
    println!("Loaded {} frame collections", frame_collections.len());
    frame_collections
}
