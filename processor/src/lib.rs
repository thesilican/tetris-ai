#![feature(once_cell)]
mod frames;
mod game_ext;
pub mod replay;
mod training_data;

pub use frames::FrameCollection;
pub use game_ext::{GameAction, GameExt};
pub use replay::Replay;
