use pc_finder::*;
use rand::prelude::*;

// Roughly count the upper limit of number of valid boards
// by randomly selecting boards and checking if they are valid
fn main() {
    let mut valid = 0;
    let mut rng = StdRng::seed_from_u64(0);
    for count in 0.. {
        let num = rng.next_u64();
        let board = PcBoard::from_u64(num);
        if board.is_valid() {
            valid += 1;
        }
        if count % 10000 == 0 {
            let pow = (2.0f64.powi(40)) * (valid as f64) / (count as f64) / 1_000_000.0;
            println!("{:>15} / {:<15} = {:0.4}m", valid, count, pow);
            println!("{}", board)
        }
    }
}
