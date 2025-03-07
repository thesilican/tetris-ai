use libtetris::Ai;
use tree_bot::{TreeAi, DEFAULT_SCORE_PARAMS};

fn main() {
    TreeAi::new(4, 4, DEFAULT_SCORE_PARAMS).demo();
}
