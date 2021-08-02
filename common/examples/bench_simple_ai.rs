use common::api::{Ai, SimpleAi};

fn main() {
    SimpleAi::new().bench_ai(10_000, 0);
}
