use pc_finder::PcBoard;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

fn main() {
    // static EXIT: AtomicBool = AtomicBool::new(false);
    // static COUNT: AtomicU64 = AtomicU64::new(0);
    // static VALID: AtomicU64 = AtomicU64::new(0);
    // ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();
    // let threads = (0..20)
    //     .map(|i| {
    //         std::thread::spawn(move || {
    //             let mut rng = StdRng::seed_from_u64(i);
    //             while !EXIT.load(Ordering::Relaxed) {
    //                 let num = rng.next_u64();
    //                 COUNT.fetch_add(1, Ordering::Relaxed);
    //                 if PcBoard::from_u64(num).is_valid() {
    //                     VALID.fetch_add(1, Ordering::Relaxed);
    //                 }
    //             }
    //         })
    //     })
    //     .collect::<Vec<_>>();
    // while !EXIT.load(Ordering::Relaxed) {
    //     std::thread::sleep(std::time::Duration::from_millis(100));
    //     let count = COUNT.load(Ordering::Relaxed);
    //     let valid = VALID.load(Ordering::Relaxed);
    //     let pow = -((valid as f64) / (count as f64)).log2();
    //     println!("{:>15} / {:<15} = 2^-{:0.4}", valid, count, pow);
    // }
    // for thread in threads {
    //     thread.join().unwrap();
    // }

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
