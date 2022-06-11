use common::*;
use pc_finder::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> GenericResult<()> {
    // Prune boards that have backlinks from parent
    // Ensure that stack and visited are empty before starting prune
    static EXIT: AtomicBool = AtomicBool::new(false);
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    let mut visited = load_visited()?;
    let mut stack = load_stack()?;
    stack.push(PcBoard::new());
    while let Some(board) = stack.pop() {
        if visited.contains(&board) {
            continue;
        }
        visited.insert(board);
        println!(
            "{}\n  Stack: {:>8}\nVisited: {:>8}\n",
            board,
            stack.len(),
            visited.len()
        );
        let parents = fetch_parents(board)?;
        for parent in parents {
            if visited.contains(&parent) {
                continue;
            }
            stack.push(parent);
        }
        if EXIT.load(Ordering::Relaxed) {
            break;
        }
    }
    if stack.len() == 0 {
        println!("Finished!");
    }
    save_stack(&stack)?;
    save_visited(&visited)?;
    Ok(())
}
