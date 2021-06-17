use crate::model::piece::Piece;

use super::game::{Game, GameMove};
use lazy_static::lazy_static;
use std::collections::HashMap;

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
    perm
}
lazy_static! {
    // Precomputed move permutations
    // Double-rotate permutations: 1144
    static ref PERMS_DR: Vec<Vec<GameMove>> = {
        let mut perms = Vec::new();
        for rot_1 in 0..4 {
            for rot_2 in 0..4 {
                for hold in &[false, true] {
                    for rot in 0..4 {
                        for shift in -4..=5 {
                            if rot_1 == 0 && rot_2 != 0 {
                                continue;
                            }
                            let perm = gen_move_vec(*hold, rot, shift, rot_1, rot_2);
                            perms.push(perm);
                        }
                    }
                }
            }
        }
        perms
    };
    // TODO: Add single-final-rotation and no-final-rotation permutations
}

fn gen_child_states(
    game: &Game,
    perms: &'static Vec<Vec<GameMove>>,
) -> Vec<(Game, &'static [GameMove])> {
    assert!(
        game.queue_pieces.len() >= 2 || (game.hold_piece.is_some() && game.queue_pieces.len() >= 1)
    );
    let current_piece = game.current_piece;
    let hold_piece = Piece::from(game.hold_piece.unwrap_or(game.queue_pieces[0]));
    // Map to remove duplicates
    let mut map = HashMap::<_, &'static [GameMove]>::new();
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
                GameMove::Hold => {}
                GameMove::HardDrop => {
                    panic!("perms should not have HardDrop")
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
        // Get canonical key
        let key = {
            let mut y = piece.location.1;
            let mut shape = *piece.get_bit_shape(None, None);
            // Canonicalize by shifting shape down until bottom row is not empty
            while shape[0] == 0 {
                for i in 0..3 {
                    shape[i] = shape[i + 1];
                }
                shape[3] = 0;
                y += 1;
            }
            (shape, y)
        };
        map.entry(key)
            .and_modify(|val| {
                // Keep the shorter permutation
                if val.len() > perm.len() {
                    *val = perm
                }
            })
            .or_insert(perm);
    }
    map.into_iter()
        .map(|(_, v)| {
            // TODO: could speed up by using existing piece
            let mut game = game.clone();
            for game_move in v {
                game.make_move(*game_move);
            }
            game.make_move(GameMove::HardDrop);
            (game, v)
        })
        .collect()
}

pub fn gen_child_states_dr(game: &Game) -> Vec<(Game, &'static [GameMove])> {
    gen_child_states(game, &PERMS_DR)
}
