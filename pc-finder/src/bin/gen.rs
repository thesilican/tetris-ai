#![feature(once_cell)]
use common::*;
use pc_finder::*;
use std::{
    lazy::SyncLazy,
    sync::atomic::{AtomicBool, Ordering},
};

fn board_fits_tess(board: PcBoard, tess: Tess) -> bool {
    #[inline]
    fn fits(test: [u16; 4], tess: Tess) -> bool {
        // Bits from board and inverted board masked by piece shape
        // Fails if there are both board bits and inverted board bits
        // otherwise succeeds
        for mask in tess.pieces.iter().map(|&x| x.rows) {
            let mut test_normal = test;
            for i in 0..4 {
                test_normal[i] &= mask[i];
            }
            let normal = test_normal.iter().any(|&x| x != 0);
            let mut test_invert = test;
            for i in 0..4 {
                test_invert[i] = !test_invert[i] & mask[i];
            }
            let invert = test_invert.iter().any(|&x| x != 0);
            if normal && invert {
                return false;
            }
        }
        true
    }

    // Number of empty rows
    let clear_rows = board.rows.iter().filter(|&&x| x == 0).count();
    const FULL_ROW: u16 = (1 << BOARD_WIDTH) - 1;
    static PERMS: SyncLazy<Vec<Vec<[usize; 4]>>> = SyncLazy::new(|| {
        vec![
            // 0
            vec![[0, 1, 2, 3]],
            // 1
            vec![[4, 0, 1, 2], [0, 4, 1, 2], [0, 1, 4, 2], [0, 1, 2, 4]],
            // 2
            vec![
                [4, 4, 0, 1],
                [4, 0, 4, 1],
                [4, 0, 1, 4],
                [0, 4, 4, 1],
                [0, 4, 1, 4],
                [0, 0, 4, 4],
            ],
            // 3
            vec![[4, 4, 4, 0], [4, 0, 4, 4], [4, 4, 0, 4], [4, 4, 4, 0]],
            // 4
            vec![],
        ]
    });
    // Slice perms up to the number of empty rows
    for perms in &PERMS[0..=clear_rows] {
        for &perm in perms {
            let get_row = |i: usize| {
                if perm[i] == 4 {
                    FULL_ROW
                } else {
                    board.rows[perm[i]]
                }
            };
            // Test is board.rows but with possible inserted full rows
            let test = [get_row(0), get_row(1), get_row(2), get_row(3)];
            if fits(test, tess) {
                return true;
            }
        }
    }
    false
}

fn main() -> GenericResult<()> {
    // Use DFS to generate all child boards
    static EXIT: AtomicBool = AtomicBool::new(false);
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    let tessellations = load_tessellations()?;
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
        let children = board.child_boards();
        let mut good_children = Vec::new();
        for &child in children.iter() {
            if visited.contains(&child) {
                continue;
            }
            if !tessellations
                .iter()
                .any(|&tess| board_fits_tess(child, tess))
            {
                continue;
            }
            stack.push(child);
            good_children.push(child);
        }
        record_parent_children(board, &good_children)?;
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
