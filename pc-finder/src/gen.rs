use crate::*;
use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

#[derive(Debug, Clone)]
struct BoardInfo {
    pub children: Vec<PcBoardSer>,
    pub backlinks: Vec<PcBoardSer>,
    pub visited: bool,
}
impl BoardInfo {
    fn new() -> Self {
        BoardInfo {
            children: Vec::new(),
            backlinks: Vec::new(),
            visited: false,
        }
    }
}

fn get_board_from_num(num: u64) -> Option<PcBoardSer> {
    let bits = num.to_le_bytes();
    let pc_board_ser = PcBoardSer::new(bits[0..5].try_into().unwrap());
    if PcBoard::from(pc_board_ser).is_valid() {
        Some(pc_board_ser)
    } else {
        None
    }
}

fn gen_valid_boards() -> HashMap<PcBoardSer, BoardInfo> {
    let mut map = HashMap::new();
    for num in 0..(2u64.pow(40)) {
        if let Some(pc_board_ser) = get_board_from_num(num) {
            map.insert(pc_board_ser, BoardInfo::new());
        }
    }
    map
}

pub fn count_boards() {
    println!("Starting to counting boards");

    let mut valid_boards = gen_valid_boards();
    println!("Generated {} valid boards", valid_boards.len());

    // Generate forward links
    let count = AtomicUsize::new(0);
    valid_boards.par_iter_mut().for_each(|(board, info)| {
        let children = PcBoard::from(*board).child_boards().map(PcBoardSer::from);
        info.children.extend(children);
        count.fetch_add(info.children.len(), Ordering::Relaxed);
    });
    println!("Generated {} forward links", count.load(Ordering::Relaxed));

    // Generate backlinks
    // Copy over keys
    let mut count = 0;
    let keys = valid_boards.keys().map(|x| *x).collect::<Vec<_>>();
    let mut children = Vec::<PcBoardSer>::new();
    for key in keys.iter() {
        // Copy children
        children.clear();
        children.extend_from_slice(&valid_boards.get(key).unwrap().children);
        for child in children.iter() {
            if let Some(info) = valid_boards.get_mut(child) {
                info.backlinks.push(*key);
                count += 1;
            }
        }
    }
    println!("Generated {} backlinks", count);

    // DFS over backlinks
    let initial = PcBoardSer::from(PcBoard::new());
    let mut stack = vec![initial];
    valid_boards.get_mut(&initial).unwrap().visited = true;
    while let Some(board) = stack.pop() {
        let backlinks = valid_boards
            .get(&board)
            .expect("expected board to be in valid_boards")
            .backlinks
            .clone();
        for child in backlinks {
            let info = valid_boards
                .get_mut(&child)
                .expect("expected child board to be in valid_boards");
            if !info.visited {
                stack.push(child);
                info.visited = true;
            }
        }
    }

    let total_visited = valid_boards
        .into_iter()
        .filter(|(_, info)| info.visited)
        .count();
    println!("DFS visited {} boards", total_visited);
}
