#![feature(once_cell)]
mod gen;

// use common::*;
pub use gen::*;
// use std::collections::VecDeque;

// pub struct PCAi;

// impl PCAi {
//     pub fn new() -> Self {
//         PCAi
//     }
// }

// impl Ai for PCAi {
//     fn evaluate(&mut self, game: &Game) -> AiRes {
//         let res = self.find(*game, 10);
//         match res {
//             Some(moves) => {
//                 let moves = moves.into_iter().fold(Vec::new(), |mut a, v| {
//                     a.extend(v);
//                     a
//                 });
//                 AiRes::Success { moves, score: None }
//             }
//             None => AiRes::Fail {
//                 reason: "Could not find PC solution".into(),
//             },
//         }
//     }
// }
// impl PCAi {
//     fn find(&self, game: Game, depth: i32) -> Option<VecDeque<&[GameMove]>> {
//         println!("{}", game);
//         if depth == 0 {
//             return None;
//         }
//         let child_states = game.child_states(&MOVES_1F);
//         // Filter child states that are greater than 5 blocks
//         let child_states = child_states
//             .into_iter()
//             .filter(|child_state| child_state.game.board.matrix[4] == 0)
//             .collect::<Vec<_>>();
//         // First check if any are PCs
//         for ChildState { game, moves } in child_states.iter() {
//             if game.board.matrix[0] == 0 {
//                 return Some(VecDeque::from([*moves]));
//             }
//         }
//         // DFS through child states
//         for ChildState { game, moves } in child_states {
//             let res = self.find(game, depth - 1);
//             if let Some(mut found_moves) = res {
//                 found_moves.push_front(moves);
//                 return Some(found_moves);
//             }
//         }
//         None
//     }
// }
