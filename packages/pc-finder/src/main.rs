use common::misc::GenericErr;
use common::model::game::{Game, GameMove};
use common::model::piece::{Piece, PieceType};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::{stdin, stdout, Write};

lazy_static! {
    static ref PERMUTATIONS: Vec<Vec<GameMove>> = {
        fn rot_to_move(rot: i32) -> Vec<GameMove> {
            match rot {
                0 => vec![],
                1 => vec![GameMove::RotateRight],
                2 => vec![GameMove::Rotate180],
                3 => vec![GameMove::RotateLeft],
                _ => unreachable!(),
            }
        }
        let mut perms = Vec::new();
        for hold in &[false, true] {
            for rot_0 in 0..4 {
                for shift in -3..=3i32 {
                    for rot_1 in 0..4 {
                        for rot_2 in 0..4 {
                            if rot_1 == 0 && rot_2 != 0 {
                                continue;
                            }
                            let mut perm = Vec::new();
                            match *hold {
                                false => {}
                                true => perm.push(GameMove::Hold),
                            };
                            perm.extend(&rot_to_move(rot_0));
                            for _ in 0..shift.abs() {
                                if shift < 0 {
                                    perm.push(GameMove::ShiftLeft);
                                } else {
                                    perm.push(GameMove::ShiftRight);
                                }
                            }
                            perm.push(GameMove::SoftDrop);
                            perm.extend(&rot_to_move(rot_1));
                            perm.extend(&rot_to_move(rot_2));
                            perm.push(GameMove::SoftDrop);
                            perms.push(perm);
                        }
                    }
                }
            }
        }
        perms
    };
}

fn generate_child_states(game: &Game) -> Vec<(Game, Vec<GameMove>)> {
    let mut empty_row = 0;
    for i in 0..=4 {
        if game.board.matrix[i] == 0 {
            empty_row = i;
            break;
        }
    }
    // De-duplicate child states
    let mut map = HashMap::new();
    for perm in &*PERMUTATIONS {
        let mut game = game.clone();
        let mut hold = false;
        for game_move in perm {
            if let GameMove::Hold = game_move {
                hold = true;
            }
            game.make_move(*game_move);
        }
        game.make_move(GameMove::HardDrop);
        if game.board.matrix[empty_row] != 0 {
            continue;
        }
        map.insert((game.board.clone(), hold), (game.clone(), perm.clone()));
    }
    map.into_iter().map(|(_, v)| v).collect::<Vec<_>>()
}

fn solve_pc(game: &Game) -> Option<Vec<GameMove>> {
    if game.board.matrix[0] == 0 {
        return Some(vec![]);
    }
    if game.queue_pieces.len() == 0 {
        return None;
    }
    for (child_game, mut moves) in generate_child_states(&game) {
        // println!("{}", child_game);
        let res = solve_pc(&child_game);
        if let Some(child_moves) = res {
            moves.push(GameMove::HardDrop);
            moves.extend(&child_moves);
            return Some(moves);
        }
    }
    None
}

fn get_input() -> Result<(bool, Vec<PieceType>), GenericErr> {
    let mut buffer = String::new();
    let left = loop {
        print!("Direction [L/R]: ");
        stdout().flush().unwrap();
        stdin().read_line(&mut buffer).unwrap();
        match &*buffer.trim().to_ascii_uppercase() {
            "L" => break true,
            "R" => break false,
            _ => {}
        }
    };
    let mut queue = {
        print!("Queue [O/I/T/L/J/S/Z]: ");
        stdout().flush().unwrap();
        buffer.clear();
        stdin().read_line(&mut buffer).unwrap();
        let mut queue = Vec::new();
        for piece_char in buffer.trim().chars() {
            let piece_type = match PieceType::try_from(piece_char) {
                Ok(piece) => piece,
                Err(_) => panic!(),
            };
            queue.push(piece_type)
        }
        queue
    };
    assert!(queue.len() <= 4);
    // Dud
    queue.push(PieceType::O);
    Ok((left, queue))
}

fn main() -> Result<(), GenericErr> {
    loop {
        // Get Input
        let (left, mut queue) = get_input()?;
        println!("\nDirection: {}", if left { "left" } else { "right" });
        println!(
            "Queue: {}",
            queue
                .iter()
                .map(|x| String::from(x.to_char()))
                .reduce(|mut acc, v| {
                    acc.push(' ');
                    acc.push_str(&v);
                    acc
                })
                .unwrap()
        );
        println!("Solving...");

        // Build game
        let mut game = Game::new();
        game.current_piece.piece_type = queue.remove(0);
        game.extend_queue(&queue);
        if left {
            game.board.set_matrix([
                0b0000001110001111,
                0b0000001111001111,
                0b0000001110001111,
                0b0000001100001111,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]);
        } else {
            game.board.set_matrix([
                0b0000001111000111,
                0b0000001111001111,
                0b0000001111000111,
                0b0000001111000011,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ]);
        }
        let solution = solve_pc(&game);
        if let None = solution {
            println!("No solutions found :(");
        } else if let Some(moves) = solution {
            println!("{}", game);
            for game_move in moves {
                if let GameMove::HardDrop = game_move {
                    println!("{}", game);
                }
                game.make_move(game_move);
            }
        }
    }
}
