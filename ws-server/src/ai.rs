use common::api::{Ai, SimpleAi};

pub fn get_ai() -> Box<dyn Ai> {
    let ai = SimpleAi::new();
    Box::new(ai)
}
