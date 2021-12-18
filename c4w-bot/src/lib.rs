#![feature(once_cell)]
mod center_state;

pub use center_state::*;
use common::*;

pub struct C4WBot {}
impl C4WBot {
    pub fn new() -> Self {
        C4WBot {}
    }
}
impl Ai for C4WBot {
    fn evaluate(&mut self, _game: &Game) -> AiRes {
        todo!()
    }
}
