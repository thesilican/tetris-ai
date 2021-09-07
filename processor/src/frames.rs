use common::model::*;
use std::{ffi::OsStr, fs};

#[derive(Debug, Clone)]
pub struct FrameCollection {
    pub name: String,
    pub frames: Vec<Game>,
}

impl FrameCollection {
    pub fn load() -> Vec<FrameCollection> {
        println!("Loading frames...");
        let mut frame_collections = Vec::new();
        let paths = fs::read_dir("data/frames").unwrap();
        for path in paths {
            let filename = path.unwrap().path();
            if filename.extension() != Some(OsStr::new("json")) {
                continue;
            }
            let text = fs::read_to_string(&filename).unwrap();
            let frames = serde_json::from_str(&text).unwrap();
            let frame_collection = FrameCollection {
                name: filename.file_stem().unwrap().to_str().unwrap().into(),
                frames,
            };
            frame_collections.push(frame_collection);
        }
        // Sort by name alphabetically
        frame_collections.sort_by(|a, b| a.name.cmp(&b.name));
        println!("Loaded {} frame collections", frame_collections.len());
        frame_collections
    }
}
