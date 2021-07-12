use common::api::{SimpleAi, TetrisAi};

pub fn get_ai() -> Box<dyn TetrisAi> {
    let ai = SimpleAi::new();
    Box::new(ai)
}
