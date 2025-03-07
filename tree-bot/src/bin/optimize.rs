use tree_bot::Optimizer;

fn main() {
    let mut optimizer = Optimizer::new(0);
    optimizer.init();
    for _ in 0..100 {
        optimizer.perform_epoch();
    }
}
