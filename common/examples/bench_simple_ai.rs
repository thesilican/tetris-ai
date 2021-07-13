use common::api::{SimpleAi, TetrisAi};

fn main() {
    SimpleAi::new().bench_ai(10_000, 0);
}
