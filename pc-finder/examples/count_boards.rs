use pc_finder::PcBoard;
use rand::prelude::*;

fn main() {
    let mut valid = 0;
    let mut rng = StdRng::seed_from_u64(0);
    for count in 0.. {
        let num = rng.next_u64();
        let board = PcBoard::from_u64(num);
        if board.is_valid() {
            valid += 1;
        }
        if count % 1000 == 0 {
            let pow = -((valid as f64) / (count as f64)).log2();
            println!("{:>15} / {:<15} = 2^-{:0.4}", valid, count, pow);
            println!("{}", board)
        }
    }
}
