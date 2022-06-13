use common::*;
use pc_finder::*;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() -> GenericResult<()> {
    // Prune boards that have backlinks from parent
    // Ensure that stack and visited are empty before starting prune
    static EXIT: AtomicBool = AtomicBool::new(false);
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    let mut pruned = load_pruned()?;
    let mut stack = load_stack()?;
    stack.push(PcBoard::new());
    while let Some(board) = stack.pop() {
        if pruned.contains(&board) {
            continue;
        }
        pruned.insert(board);
        println!(
            "{}\n  Stack: {:>8}\nVisited: {:>8}\n",
            board,
            stack.len(),
            pruned.len()
        );
        let parents = fetch_parents(board)?;
        for parent in parents {
            if pruned.contains(&parent) {
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
    save_pruned(&pruned)?;
    Ok(())
}
