use common::api::Ai;
use deep_bot::DeepAi;

fn main() {
    DeepAi::new(4, 20).bench_ai(1, 0);
}
