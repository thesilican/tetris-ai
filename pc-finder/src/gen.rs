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

pub fn count_boards() {
    println!("Starting to counting boards");

    // Generate valid PcBoards
    let mut valid_boards = (0..(2u64).pow(20))
        .into_par_iter()
        .filter_map(|num| {
            let board = PcBoardSer::from_u64(num);
            if PcBoard::from(board).is_valid() {
                Some((board, BoardInfo::new()))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();
    println!("Generated {} valid boards", valid_boards.len());

    // Generate forward links (children)
    let count = AtomicUsize::new(0);
    valid_boards.par_iter_mut().for_each(|(board, info)| {
        let children = PcBoard::from(*board).child_boards().map(PcBoardSer::from);
        info.children.extend(children);
        count.fetch_add(info.children.len(), Ordering::Relaxed);
    });
    println!("Generated {} forward links", count.load(Ordering::Relaxed));

    // Generate backlinks
    let mut count = 0;
    let keys = valid_boards.keys().map(|x| *x).collect::<Vec<_>>();
    for key in keys.iter() {
        let children = valid_boards.get(key).unwrap().children.clone();
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

    let final_boards = valid_boards
        .into_iter()
        .filter_map(|(board, info)| {
            if info.visited {
                Some(board.to_u64())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("DFS visited {} boards", final_boards.len());
    let file = std::fs::File::create("out.json").expect("error creating file");
    serde_json::to_writer(file, &*final_boards).expect("error writing to file");
}
