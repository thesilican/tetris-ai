use common::model::*;
use serde::Deserialize;
use std::fs;

#[derive(Debug)]
pub struct FrameCollection {
    pub name: String,
    pub frames: Vec<Frame>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "FrameJson")]
pub struct Frame {
    board: [u16; BOARD_HEIGHT as usize],
    active: [u16; BOARD_HEIGHT as usize],
    hold: Option<PieceType>,
    can_hold: bool,
    queue: Vec<PieceType>,
}

// Utility for deserializing frame JSONs
#[derive(Debug, Deserialize)]
struct FrameJson {
    board: [[i8; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize],
    active: [[i8; BOARD_HEIGHT as usize]; BOARD_WIDTH as usize],
    hold: (Option<PieceType>, bool),
    queue: Vec<PieceType>,
}
impl From<FrameJson> for Frame {
    fn from(frame_json: FrameJson) -> Self {
        let mut board = [0; BOARD_HEIGHT as usize];
        let mut active = [0; BOARD_HEIGHT as usize];
        for i in 0..BOARD_WIDTH as usize {
            for j in 0..BOARD_HEIGHT as usize {
                if frame_json.active[i][j] == 1 {
                    active[j] |= 1 << i;
                }
                if frame_json.board[i][j] == 1 {
                    board[j] |= 1 << i;
                }
            }
        }
        Frame {
            board,
            active,
            hold: frame_json.hold.0,
            queue: frame_json.queue,
            can_hold: frame_json.hold.1,
        }
    }
}

pub fn load_frames() -> Vec<FrameCollection> {
    let paths = fs::read_dir("data/frames").unwrap();
    let mut frame_collections = Vec::new();
    for path in paths {
        let path = path.unwrap();
        let file_name = path.path();
        let text = fs::read_to_string(file_name).unwrap();
        let frames = serde_json::from_str::<Vec<Frame>>(&text).unwrap();
        frame_collections.push(FrameCollection {
            name: path.file_name().to_str().unwrap().to_string(),
            frames,
        });
    }
    frame_collections
}
