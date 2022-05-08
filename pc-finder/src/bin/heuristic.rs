use fnv::FnvHashSet;
use pc_finder::*;
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::<PcBoard>::new();
    let mut visited = FnvHashSet::<PcBoard>::default();
    queue.push_back(PcBoard::new());
    let mut counter = 0;
    while let Some(board) = queue.pop_front() {
        visited.insert(board);
        for child in board.child_boards() {
            if visited.insert(child) {
                queue.push_back(child);
            }
        }
        counter += 1;
        if counter % 123 == 0 {
            println!("{}\n", board);
        }
    }
}
