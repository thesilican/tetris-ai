use common::model::PieceType;

use crate::{frames::FrameCollection, game_ext::LongQueue, GameAction};
use std::{collections::VecDeque, iter::FromIterator};

#[derive(Debug, Clone)]
pub struct Replay {
    name: String,
    queue: LongQueue,
    actions: Vec<GameAction>,
}

pub fn frames_to_queue(frames: &FrameCollection) -> LongQueue {
    // Each frame contains 5 queues
    let mut pieces = Vec::new();
    // Start with the first frame
    let mut prev_queue = [PieceType::O; 5];

    for frame in &frames.frames {}

    LongQueue::from_iter(pieces)
}

pub fn frames_to_replay(frames: &FrameCollection) -> Replay {
    // Start by determining the queue
    let queue = frames_to_queue(frames);
    todo!()
}
