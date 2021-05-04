use crate::ai::consts::CENTER_COL;
use crate::ai::consts::CENTER_TRANSITION_HEIGHT;
use crate::ai::consts::CENTER_TRANSITION_WIDTH;
use crate::ai::consts::LEFT_COL;
use crate::ai::consts::LR_MAX_DIFF;
use crate::ai::consts::LR_WIDTH;
use crate::ai::consts::RIGHT_COL;
use crate::model::board::Board;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_NUM_ROTATION;
use crate::model::consts::PIECE_STARTING_COLUMN;
use crate::model::game::GameMove;
use crate::model::piece::Piece;
use crate::model::piece::PieceMove;
use crate::model::piece::PieceType;
use lazy_static::lazy_static;
use std::cmp::max;
use std::cmp::min;
use std::cmp::Eq;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub type TransitionState<T> = HashMap<T, StateTransitionInfo<T>>;
#[derive(Debug)]
pub struct StateTransitionInfo<T> {
    pub total: i32,
    pub transitions: HashMap<PieceType, PieceTransitionInfo<T>>,
}
impl<T> StateTransitionInfo<T> {
    fn new(transitions: HashMap<PieceType, PieceTransitionInfo<T>>) -> Self {
        let mut total = 0;
        for (_, piece_transition) in transitions.iter() {
            total += piece_transition.total;
        }
        StateTransitionInfo { total, transitions }
    }
}
#[derive(Debug)]
pub struct PieceTransitionInfo<T> {
    pub total: i32,
    pub transitions: HashMap<T, Vec<GameMove>>,
}
impl<T: Hash + Eq> PieceTransitionInfo<T> {
    fn new(transitions: HashMap<T, Vec<PieceMove>>) -> Self {
        let total = transitions.len() as i32;
        let transitions = transitions
            .into_iter()
            .map(|(state, arr)| (state, arr.into_iter().map(|x| x.to_game_move()).collect()))
            .collect();
        PieceTransitionInfo { total, transitions }
    }
}

// Given a seed state and a function for generating child states,
// Generate a state transition digraph
// Then prune it until it is strongly connected
fn gen_transition_states<T: Debug + Clone + Eq + Hash>(
    seed_state: T,
    get_state_transitions: fn(state: &T) -> StateTransitionInfo<T>,
) -> TransitionState<T> {
    let mut transitions: TransitionState<T> = HashMap::new();
    // DFS throught all states
    {
        let mut active_states: Vec<T> = Vec::new();
        let mut visited_states: HashSet<T> = HashSet::new();
        active_states.push(seed_state.clone());
        visited_states.insert(seed_state.clone());

        while active_states.len() > 0 {
            let state = active_states.pop().unwrap();
            let state_transitions = get_state_transitions(&state);
            for (_, piece_transitions) in state_transitions.transitions.iter() {
                for (child_state, _) in piece_transitions.transitions.iter() {
                    if !visited_states.contains(&child_state) {
                        active_states.push(child_state.clone());
                        visited_states.insert(child_state.clone());
                    }
                }
            }
            transitions.insert(state, state_transitions);
        }
    }
    // Prune state transitions
    {
        // Reverse map from child_state -> parent_state
        let mut rev_map: HashMap<T, Vec<T>> = HashMap::new();
        for (state, state_transitions) in transitions.iter() {
            for (_, piece_transitions) in state_transitions.transitions.iter() {
                for (child_state, _) in piece_transitions.transitions.iter() {
                    match rev_map.get_mut(child_state) {
                        Some(arr) => {
                            arr.push(state.clone());
                        }
                        None => {
                            rev_map.insert(child_state.clone(), vec![state.clone()]);
                        }
                    }
                }
            }
        }

        // DFS through reverse map, to find all states that have seed_state as a child
        // (We assume that the seed state can reach itself)
        let mut visited_states: HashSet<T> = HashSet::new();
        let mut active_states: Vec<T> = Vec::new();
        active_states.push(seed_state);
        while active_states.len() > 0 {
            let state = active_states.pop().unwrap();
            visited_states.insert(state.clone());
            for parent_state in rev_map.get(&state).unwrap() {
                if !visited_states.contains(parent_state) {
                    active_states.push(parent_state.clone());
                }
            }
        }
        // Prune unvisited states
        for (_, state_transitions) in transitions.iter_mut() {
            for (_, piece_transitions) in state_transitions.transitions.iter_mut() {
                piece_transitions
                    .transitions
                    .retain(|child_state, _| visited_states.contains(child_state));
                piece_transitions.total = piece_transitions.transitions.len() as i32;
            }
            state_transitions
                .transitions
                .retain(|_, piece_transitions| piece_transitions.total != 0);
            state_transitions.total = state_transitions
                .transitions
                .iter()
                .fold(0, |acc, (_, info)| acc + info.total);
        }
        transitions.retain(|state, state_transitions| {
            visited_states.contains(state) && state_transitions.total != 0
        });
    }
    transitions
}

// Generates a list of all possible piece moves
// Including a final rotation
fn gen_piece_moves(piece: &Piece) -> Vec<Vec<PieceMove>> {
    // TODO:
    // Maybe optimize for piece columns
    let mut ret = Vec::new();
    for rotation in 0..PIECE_NUM_ROTATION {
        let (left_bound, right_bound, _, _) = piece.get_shift_bounds(Some(rotation));
        let left_bound = left_bound - PIECE_STARTING_COLUMN;
        let right_bound = right_bound - PIECE_STARTING_COLUMN;
        for shift in left_bound..=right_bound {
            for final_rotation in 0..PIECE_NUM_ROTATION {
                let mut moves = Vec::new();
                match rotation {
                    0 => (),
                    1 => moves.push(PieceMove::RotateRight),
                    2 => moves.push(PieceMove::Rotate180),
                    3 => moves.push(PieceMove::RotateLeft),
                    _ => panic!(),
                }
                for _ in 0..(shift.abs()) {
                    if shift < 0 {
                        moves.push(PieceMove::ShiftLeft);
                    } else {
                        moves.push(PieceMove::ShiftRight);
                    }
                }
                match final_rotation {
                    0 => (),
                    1 => {
                        moves.push(PieceMove::SoftDrop);
                        moves.push(PieceMove::RotateRight);
                    }
                    2 => {
                        moves.push(PieceMove::SoftDrop);
                        moves.push(PieceMove::Rotate180);
                    }
                    3 => {
                        moves.push(PieceMove::SoftDrop);
                        moves.push(PieceMove::RotateLeft);
                    }
                    _ => panic!(),
                }
                ret.push(moves);
            }
        }
    }
    ret
}

lazy_static! {
    pub static ref C4W_TRANSITIONS: C4WTransitions = C4WTransitions::new();
}
#[derive(Debug)]
pub struct C4WTransitions {
    pub center: TransitionState<u16>,
    pub left: TransitionState<(i8, i8, i8)>,
    pub right: TransitionState<(i8, i8, i8)>,
}
impl C4WTransitions {
    fn new() -> Self {
        let start = std::time::Instant::now();

        // Corresponds to this state:
        // ....
        // ....
        // ##..
        // #...
        let center_seed_state = 0b0000_0000_0011_0001;
        let center =
            gen_transition_states(center_seed_state, C4WTransitions::center_gen_transitions);
        let lr_seed_state = (0, 0, 0);
        let left = gen_transition_states(lr_seed_state, |state| {
            C4WTransitions::lr_gen_transitions(state, true)
        });
        let right = gen_transition_states(lr_seed_state, |state| {
            C4WTransitions::lr_gen_transitions(state, false)
        });
        let end = start.elapsed();
        eprintln!("Computed transitions in {:?}", end);
        C4WTransitions {
            center,
            left,
            right,
        }
    }

    fn center_gen_transitions(state: &u16) -> StateTransitionInfo<u16> {
        let mut board = Board::new();
        let mut transitions = HashMap::new();
        // Find all child states given a state
        for piece_type in PieceType::iter_types() {
            let mut piece = Piece::new(piece_type.clone());
            let mut piece_transitions: HashMap<u16, Vec<PieceMove>> = HashMap::new();
            for moves in gen_piece_moves(&piece) {
                C4WTransitions::center_set_state(&mut board, *state);
                piece.reset();
                for piece_move in moves.iter() {
                    piece.make_move(&board, piece_move);
                }
                piece.soft_drop(&board);
                let (res, _) = board.lock(&piece);
                if res.lines_cleared != 1 {
                    continue;
                }
                let new_state = C4WTransitions::center_get_state(&board);
                match piece_transitions.get_mut(&new_state) {
                    Some(old_moves) => {
                        // Choose the one that's shorter
                        // If they're the same length, choose the one without the soft drop
                        if old_moves.len() > moves.len() {
                            *old_moves = moves;
                        } else if old_moves.len() == moves.len() {
                            if old_moves.contains(&PieceMove::SoftDrop)
                                && !moves.contains(&PieceMove::SoftDrop)
                            {
                                *old_moves = moves;
                            }
                        }
                    }
                    None => {
                        piece_transitions.insert(new_state, moves);
                    }
                };
            }
            transitions.insert(piece_type, PieceTransitionInfo::new(piece_transitions));
        }
        StateTransitionInfo::new(transitions)
    }
    pub fn center_get_state(board: &Board) -> u16 {
        let mut state = 0;
        for j in (0..CENTER_TRANSITION_HEIGHT).rev() {
            let mut row = board.matrix[j as usize] >> CENTER_COL;
            row &= 0b1111;
            state <<= 4;
            state |= row;
        }
        state
    }
    fn center_set_state(board: &mut Board, mut state: u16) {
        board.set_cols([20, 20, 20, 0, 0, 0, 0, 20, 20, 20]);
        for j in 0..CENTER_TRANSITION_HEIGHT {
            let row = (state & 0b1111) << CENTER_COL;
            state >>= 4;
            board.matrix[j as usize] |= row;
        }
        for i in 0..CENTER_TRANSITION_WIDTH {
            board.recalculate_metadata(i + CENTER_COL);
        }
    }

    fn lr_gen_transitions(state: &(i8, i8, i8), left: bool) -> StateTransitionInfo<(i8, i8, i8)> {
        let mut board = Board::new();
        let mut transitions = HashMap::new();
        for piece_type in PieceType::iter_types() {
            let mut piece = Piece::new(piece_type.clone());
            let mut piece_transitions = HashMap::<(i8, i8, i8), Vec<PieceMove>>::new();
            'moves: for moves in gen_piece_moves(&piece) {
                C4WTransitions::lr_set_state(&mut board, *state, left);
                piece.reset();
                for piece_move in moves.iter() {
                    piece.make_move(&board, piece_move);
                }
                piece.soft_drop(&board);

                board.lock(&piece);

                // Disallow if a piece was dropped beyond the left or right column
                for i in 0..(BOARD_WIDTH - LR_WIDTH) {
                    let col = if left {
                        (LEFT_COL + LR_WIDTH) + i
                    } else {
                        (BOARD_WIDTH - RIGHT_COL - LR_WIDTH) + i
                    };
                    if board.height_map[col as usize] != 0 {
                        continue 'moves;
                    }
                }
                // Disallow holes
                for i in 0..LR_WIDTH {
                    let col = if left { LEFT_COL + i } else { RIGHT_COL + i };
                    if board.holes[col as usize] != 0 {
                        continue 'moves;
                    }
                }

                let mut new_state = C4WTransitions::lr_get_state(&board, left);
                // Normalize, disallow if max diff is too big
                let min_height = min(new_state.0, min(new_state.1, new_state.2));
                let max_height = max(new_state.0, max(new_state.1, new_state.2));
                if max_height - min_height > LR_MAX_DIFF as i8 {
                    continue;
                }
                new_state.0 -= min_height;
                new_state.1 -= min_height;
                new_state.2 -= min_height;

                // Replace current moves if it is shorter
                match piece_transitions.get_mut(&new_state) {
                    Some(curr_moves) => {
                        if curr_moves.len() > moves.len() {
                            *curr_moves = moves;
                        }
                    }
                    None => {
                        piece_transitions.insert(new_state, moves);
                    }
                }
            }
            transitions.insert(piece_type, PieceTransitionInfo::new(piece_transitions));
        }
        StateTransitionInfo::new(transitions)
    }
    pub fn lr_get_state(board: &Board, left: bool) -> (i8, i8, i8) {
        if left {
            (
                board.height_map[LEFT_COL as usize] as i8,
                board.height_map[(LEFT_COL + 1) as usize] as i8,
                board.height_map[(LEFT_COL + 2) as usize] as i8,
            )
        } else {
            (
                board.height_map[RIGHT_COL as usize] as i8,
                board.height_map[(RIGHT_COL + 1) as usize] as i8,
                board.height_map[(RIGHT_COL + 2) as usize] as i8,
            )
        }
    }
    fn lr_set_state(board: &mut Board, state: (i8, i8, i8), left: bool) {
        let mut heights = [0; BOARD_WIDTH as usize];
        if left {
            heights[LEFT_COL as usize] = state.0 as i32;
            heights[(LEFT_COL + 1) as usize] = state.1 as i32;
            heights[(LEFT_COL + 2) as usize] = state.2 as i32;
        } else {
            heights[RIGHT_COL as usize] = state.0 as i32;
            heights[(RIGHT_COL + 1) as usize] = state.1 as i32;
            heights[(RIGHT_COL + 2) as usize] = state.2 as i32;
        }
        board.set_cols(heights);
    }
}
