use crate::*;
use common::*;
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    fs::File,
    str::FromStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
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

pub fn count_valid_boards() {
    println!("Starting to count valid boards");

    let exp = std::env::args()
        .collect::<Vec<_>>()
        .get(1)
        .map(|x| u32::from_str(&x).ok())
        .flatten()
        .unwrap_or(40);
    println!("from 0 to 2^{}", exp);
    let valid_boards = (0..(2u64).pow(exp))
        .into_par_iter()
        .filter_map(|num| {
            let board = PcBoardSer::from_u64(num);
            if PcBoard::from(board).is_valid() {
                Some(num)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    println!("Found {} valid boards", valid_boards.len());

    let file = File::create("data/count-valid-boards.json").expect("error creating file");
    serde_json::to_writer(&file, &*valid_boards).expect("error writing to file");
    println!("Written boards to data/count-valid-boards.json");
}

pub fn count_boards() {
    println!("Starting to counting boards");

    // Generate valid PcBoards
    let mut valid_boards = (0..(2u64).pow(40))
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

pub fn dfs_boards() {
    const THREAD_COUNT: usize = 64;

    fn get_children(board: PcBoardSer) -> Vec<PcBoardSer> {
        let mut children = Vec::new();
        for piece_type in PieceType::all() {
            let mut game = Game::from_pieces(piece_type, None, &[PieceType::O]);
            game.board = Board::from(PcBoard::from(board));
            let child_states = game.child_states(&FRAGMENTS);
            let boards = child_states.into_iter().filter_map(|child_state| {
                PcBoard::try_from(child_state.game.board)
                    .ok()
                    .map(PcBoardSer::from)
            });
            children.extend(boards);
        }
        children
    }

    let mut stack = Vec::<PcBoardSer>::new();
    let mut visited = HashSet::<PcBoardSer>::new();
    let initial_state = PcBoardSer::new([0; 5]);
    stack.push(initial_state);
    visited.insert(initial_state);
    // DFS until the stack has enough elements
    while stack.len() < 2 * THREAD_COUNT {
        let board = stack.pop().unwrap();
        let children = get_children(board);
        for child in children {
            if visited.insert(child) {
                stack.push(child);
            }
        }
    }

    let state_arc = Arc::new(Mutex::new((stack, visited)));
    let mut jobs = Vec::new();
    for _ in 0..THREAD_COUNT {
        let state_arc = state_arc.clone();
        let job = move || loop {
            let mut state = state_arc.lock().unwrap();
            let (stack, _) = &mut *state;
            let board = match stack.pop() {
                Some(board) => board,
                None => break,
            };
            drop(state);

            let children = get_children(board);

            let mut state = state_arc.lock().unwrap();
            let (stack, visited) = &mut *state;
            for child in children {
                if visited.insert(child) {
                    stack.push(child);
                }
            }
            println!("Stack: {} Visited: {}", stack.len(), visited.len());
            drop(state);
        };
        jobs.push(job);
    }
    let thread_pool = ThreadPool::new(THREAD_COUNT);
    thread_pool.run(jobs);
}
