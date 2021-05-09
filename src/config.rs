use common::api::ai::DummyAI;
use common::api::ai::TetrisAI;

pub fn create_ai() -> impl TetrisAI + Unpin + 'static {
    DummyAI::new()
}
