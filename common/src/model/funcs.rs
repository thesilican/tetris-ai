use super::game::{Game, GameMove};
use crate::model::piece::Piece;
use lazy_static::lazy_static;
use std::collections::HashMap;

// Given the options (hold, rot, shift, rot_1, rot_2),
// generate the corresponding Vec<GameMove>
fn gen_move_vec(hold: bool, rot: i32, shift: i32, rot_1: i32, rot_2: i32) -> Vec<GameMove> {
    fn rot_to_move(rot: i32) -> Vec<GameMove> {
        match rot {
            0 => vec![],
            1 => vec![GameMove::RotateRight],
            2 => vec![GameMove::Rotate180],
            3 => vec![GameMove::RotateLeft],
            _ => unreachable!(),
        }
    }
    let mut perm = Vec::new();
    match hold {
        false => {}
        true => perm.push(GameMove::Hold),
    };
    perm.extend(&rot_to_move(rot));
    for _ in 0..shift.abs() {
        if shift < 0 {
            perm.push(GameMove::ShiftLeft);
        } else {
            perm.push(GameMove::ShiftRight);
        }
    }
    if rot_1 != 0 || rot_2 != 0 {
        perm.push(GameMove::SoftDrop);
    }
    perm.extend(&rot_to_move(rot_1));
    perm.extend(&rot_to_move(rot_2));
    perm.push(GameMove::HardDrop);
    perm
}
lazy_static! {
    // Precomputed move permutations
    // Double-rotate permutations: 1040
    static ref PERMS_DR: Vec<Vec<GameMove>> = {
        let mut perms = Vec::new();
        for rot_1 in 0..4 {
            for rot_2 in 0..4 {
                for hold in [false, true] {
                    for rot in 0..4 {
                        for shift in -4..=5 {
                            if rot_1 == 0 && rot_2 != 0 {
                                continue;
                            }
                            let perm = gen_move_vec(hold, rot, shift, rot_1, rot_2);
                            perms.push(perm);
                        }
                    }
                }
            }
        }
        println!("{}", perms.len());
        perms
    };
    // Single-rotate permutations: 320
    static ref PERMS_SR: Vec<Vec<GameMove>> = {
        let mut perms = Vec::new();
        for rot_1 in 0..4 {
            for hold in [false, true] {
                for rot in 0..4 {
                    for shift in -4..=5 {
                        let perm = gen_move_vec(hold, rot, shift, rot_1, 0);
                        perms.push(perm);
                    }
                }
            }
        }
        println!("{}", perms.len());
        perms
    };
    // No-rotate permutations: 80
    static ref PERMS_NR: Vec<Vec<GameMove>> = {
        let mut perms = Vec::new();
        for hold in [false, true] {
            for rot in 0..4 {
                for shift in -4..=5 {
                    let perm = gen_move_vec(hold, rot, shift, 0, 0);
                    perms.push(perm);
                }
            }
        }
        println!("{}", perms.len());
        perms
    };
}

// Given a game, generate all possible child states
fn gen_child_states(
    game: &Game,
    perms: &'static Vec<Vec<GameMove>>,
) -> Vec<(Game, &'static [GameMove])> {
    assert!(
        game.queue_pieces.len() >= 2 || (game.hold_piece.is_some() && game.queue_pieces.len() >= 1)
    );
    // Check if board already topped out
    if game.board.topped_out() {
        return Vec::new();
    }

    // Get current and hold piece
    let current_piece = game.current_piece;
    let mut hold_piece = Piece::from(game.hold_piece.unwrap_or(game.queue_pieces[0]));
    hold_piece.shift_down(&game.board);

    // Return value
    let mut res = Vec::<(Game, &'static [GameMove])>::new();
    // Visited states and their index
    let mut visited = HashMap::<([u16; 4], i8), usize>::new();
    // Iterate over all game move permutations
    for perm in perms.iter() {
        // Get working piece
        let mut piece = if let Some(&GameMove::Hold) = perm.get(0) {
            hold_piece
        } else {
            current_piece
        };
        // Move piece
        for game_move in perm {
            match *game_move {
                GameMove::Hold => {
                    // Skip (should only be first)
                }
                GameMove::HardDrop => {
                    // Skip (should always be last)
                }
                GameMove::ShiftLeft => {
                    piece.shift_left(&game.board);
                }
                GameMove::ShiftRight => {
                    piece.shift_right(&game.board);
                }
                GameMove::RotateLeft => {
                    piece.rotate_left(&game.board);
                }
                GameMove::RotateRight => {
                    piece.rotate_right(&game.board);
                }
                GameMove::Rotate180 => {
                    piece.rotate_180(&game.board);
                }
                GameMove::SoftDrop => {
                    piece.soft_drop(&game.board);
                }
            }
        }
        // Soft drop (to ensure consistency)
        piece.soft_drop(&game.board);
        // Get canonical key (shape and position)
        // Canonicalize by shifting shape down until bottom row is not empty
        let mut y = piece.location.1;
        let mut shape = *piece.get_bit_shape(None, None);
        while shape[0] == 0 {
            for i in 0..3 {
                shape[i] = shape[i + 1];
            }
            shape[3] = 0;
            y += 1;
        }
        let key = (shape, y);

        if visited.contains_key(&key) {
            let index = visited[&key];
            let old_perm = &mut res[index].1;
            // Replace if current permutation is shorter
            if perm.len() < old_perm.len() {
                *old_perm = perm;
            }
        } else {
            // Get child game state
            // TODO: maybe optimize using existing child piece
            let mut game = game.clone();
            for game_move in perm {
                game.make_move(*game_move);
            }
            if !game.board.topped_out() {
                // Set visited[key] to index of state
                visited.insert(key, res.len());
                res.push((game, &perm[..]));
            }
        }
    }
    // Todo: fix problems with incorrect output (logic error)
    res
}

impl Game {
    pub fn child_states_dr(&self) -> Vec<(Game, &'static [GameMove])> {
        gen_child_states(self, &PERMS_DR)
    }
    pub fn child_states_sr(&self) -> Vec<(Game, &'static [GameMove])> {
        gen_child_states(self, &PERMS_SR)
    }
    pub fn child_states_nr(&self) -> Vec<(Game, &'static [GameMove])> {
        gen_child_states(self, &PERMS_NR)
    }
}
