use common::api::Ai;
use deep_bot::DeepAi;

fn main() {
    DeepAi::new(6, 10).bench_ai(1, 0);
}
