use pc_finder::*;

fn is_valid(num: &u64) -> bool {
    let bits = num.to_le_bytes();
    let pc_board_ser = PcBoardSer::new(bits[0..5].try_into().unwrap());
    let pc_board = PcBoard::from(pc_board_ser);
    pc_board.is_valid()
}

fn main() {
    let board_count = 2u64.pow(40);
    // println!("{}", (0..board_count).into_iter().filter(is_valid).count());
    let thread_count = 16;
    let thread_work = board_count / thread_count;
    let mut threads = Vec::new();
    for i in 0..thread_count {
        let low = thread_work * i;
        let high = thread_work * (i + 1);
        let thread = std::thread::spawn(move || (low..high).into_iter().filter(is_valid).count());
        threads.push(thread);
    }

    let total_count: usize = threads.into_iter().map(|x| x.join().unwrap()).sum();
    println!("Total count: {}", total_count);
}
