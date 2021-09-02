#![feature(once_cell)]
mod frames;
mod game_ext;
mod replay;
mod training_data;

pub use frames::{load_frame_collections, FrameCollection};
pub use game_ext::{GameAction, GameExt};
pub use replay::{frame_collection_to_replay, Replay};
