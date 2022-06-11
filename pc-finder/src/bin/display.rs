use common::*;
use pc_finder::*;
use std::collections::{HashSet, VecDeque};

fn main() -> GenericResult<()> {
    let pruned = load_visited()?;
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((0, PcBoard::new()));
    while let Some((depth, board)) = queue.pop_front() {
        if visited.contains(&board) || !pruned.contains(&board) {
            continue;
        }
        visited.insert(board);
        let children = fetch_children(board)?;
        let mut children_count = 0;
        for child in children {
            if visited.contains(&child) || !pruned.contains(&child) {
                continue;
            }
            children_count += 1;
            queue.push_back((depth + 1, child));
        }
        println!(
            "Depth: {}\n{}\nChildren: {}\n",
            depth, board, children_count
        );
    }
    println!("Total: {} boards", pruned.len());
    Ok(())
}
