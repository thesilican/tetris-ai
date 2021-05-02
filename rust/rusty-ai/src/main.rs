mod ai;
mod model;

use crate::ai::ai::AIEvaluation;
use crate::ai::ai::AIWeights;
use crate::ai::ai::AI;
use crate::ai::ai::NUM_AI_WEIGHTS;
use crate::ai::trainer::AITrainer;
use crate::ai::trainer::AI_GARBAGE_FREQ;
use crate::model::consts::BOARD_HEIGHT;
use crate::model::consts::BOARD_VISIBLE_HEIGHT;
use crate::model::consts::BOARD_WIDTH;
use crate::model::consts::PIECE_COLUMN;
use crate::model::consts::PIECE_MAX_ROTATION;
use crate::model::consts::PIECE_MAX_SIZE;
use crate::model::game::Game;
use crate::model::game::GameDropOptions;
use crate::model::game::GameRNGGenerator;
use crate::model::game::GameUndoInfo;
use crate::model::piece::Piece;
use ai_api::APIMove;
use std::env;
use std::fmt::Write;
use std::io::stdin;
use std::time::Duration;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    args.next();
    let arg = args.next();
    if let None = arg {
        panic!("Expected an argument");
    }
    let arg = arg.unwrap();
    let default = String::from("PfAwxL9BFei+hPjkvZaqnT53L7+9j6ttvIhxL78CSjy+F+sS");
    match arg.as_str() {
        "test" => test(),
        "play" => play_tetris(),
        "watch" => {
            let print = args.next().unwrap_or(String::from("false")) == "true";
            let weights = &args.next().unwrap_or(default);
            watch_ai_play_tetris(weights, print);
        }
        "train" => AITrainer::new().start(),
        "api" => {
            let print = args.next().unwrap_or(String::from("false")) == "true";
            let weights = &args.next().unwrap_or(default);
            run_api(weights, print).unwrap();
        }
        _ => panic!("Unknown argument {}", arg),
    }
}

fn play_tetris() {
    let mut game = Game::new();
    let mut piece_gen = GameRNGGenerator::new(None);
    let mut undo_info: Option<GameUndoInfo> = None;
    let mut drop_options = GameDropOptions {
        rotation: 0,
        shift: 0,
        hold: false,
    };
    game.extend_queue(piece_gen.gen_7bag());
    print_state(&game, &drop_options, None, None, None).unwrap();
    for tick in 0..100_000 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        match input.trim().as_ref() {
            "a" => drop_options.shift -= 1,
            "d" => drop_options.shift += 1,
            "w" => drop_options.rotation = (drop_options.rotation + 1) % PIECE_MAX_ROTATION,
            "s" => {
                let (_, game_undo_info) = game.drop(&drop_options).expect("Drop failed");
                undo_info = Some(game_undo_info);
                drop_options.hold = false;
                drop_options.shift = 0;
                drop_options.rotation = 0;
            }
            "c" => drop_options.hold = !drop_options.hold,
            "u" => {
                if let Some(undo_info) = undo_info {
                    game.undo(&undo_info);
                }
                undo_info = None
            }
            _ => {}
        };
        print_state(&game, &drop_options, None, None, None).unwrap();
        if game.queue_len < 7 {
            game.extend_queue(piece_gen.gen_7bag());
        }
        if tick % 10 == 1 {
            game.board.add_garbage_line(piece_gen.gen_garbage_line());
            undo_info = None;
        }
    }
}

fn watch_ai_play_tetris(weight_str: &str, step: bool) {
    let mut game = Game::new();
    let weights = AIWeights::from_string(weight_str).unwrap();
    let ai = AI::new(&weights);
    // let timestamp = Utc::now().timestamp() as u64;
    let timestamp = 0;
    let mut rng = GameRNGGenerator::new(Some(timestamp));
    game.extend_queue(rng.gen_7bag());
    let start = Instant::now();
    for tick in 0..500 {
        let instant = Instant::now();
        let ai_res = ai.evaluate(&mut game, 3);
        let ms = instant.elapsed();
        if ai_res.score == f32::NEG_INFINITY {
            print_state(&game, &ai_res.drop, Some(&ai_res), Some(&weights), Some(ms)).unwrap();
            println!("TOP OUT");
            break;
        }
        if game.queue_len < 7 {
            game.extend_queue(rng.gen_7bag());
        }
        print_state(&game, &ai_res.drop, Some(&ai_res), Some(&weights), Some(ms)).unwrap();
        game.drop(&ai_res.drop).unwrap();
        if tick % AI_GARBAGE_FREQ == 0 {
            game.board.add_garbage_line(rng.gen_garbage_line());
        }
        if step {
            stdin().read_line(&mut String::new()).unwrap();
        }
    }
    println!("Total Elapsed: {}ms", start.elapsed().as_millis());
}

fn print_state(
    game: &Game,
    drop: &GameDropOptions,
    eval: Option<&AIEvaluation>,
    weights: Option<&AIWeights>,
    ms: Option<Duration>,
) -> std::fmt::Result {
    let res = write_state(game, drop, eval, weights, ms)?;
    println!("{}", res);
    Ok(())
}

fn write_state(
    game: &Game,
    drop: &GameDropOptions,
    eval: Option<&AIEvaluation>,
    weights: Option<&AIWeights>,
    ms: Option<Duration>,
) -> Result<String, std::fmt::Error> {
    let mut res = String::new();
    let piece = if drop.hold {
        game.hold.clone()
    } else {
        game.get_curr_piece().unwrap().clone()
    };
    let shape = (*Piece::info_shape(&piece, drop.rotation)).clone();

    let left = PIECE_COLUMN + drop.shift;
    for y in (0..PIECE_MAX_SIZE).rev() {
        for x in 0..BOARD_WIDTH {
            if x < left || x >= left + PIECE_MAX_SIZE {
                write!(res, "░░")?;
            } else {
                if shape[(x - left) as usize][y as usize] {
                    write!(res, "██")?;
                } else {
                    write!(res, "░░")?;
                }
            }
        }
        write!(res, "\n")?;
    }
    write!(res, "{}\n", game.board)?;
    for x in 0..BOARD_WIDTH {
        write!(res, "{: >2}", game.board.height_map[x as usize])?;
    }
    writeln!(res)?;
    write!(
        res,
        "[Game]\tPiece: {} Queue: {} {} {} {} Hold: {}\n",
        game.get_curr_piece().unwrap().to_char(),
        game.get_queue_piece(1).unwrap().to_char(),
        game.get_queue_piece(2).unwrap().to_char(),
        game.get_queue_piece(3).unwrap().to_char(),
        game.get_queue_piece(4).unwrap().to_char(),
        game.hold.to_char()
    )?;
    write!(res, "\tHoles: {} Score: {}\n", game.board.holes, game.score)?;
    if let Some(eval) = eval {
        write!(res, "[AI]\tEval: {} ", eval.score)?;
        write!(
            res,
            "Drop: {} {} {}\n",
            eval.drop.hold, eval.drop.rotation, eval.drop.shift
        )?;
    }
    if let Some(ms) = ms {
        write!(res, "\tTime: {}ms\n", ms.as_millis())?;
    }
    if let Some(weights) = weights {
        let mut weight_str = String::new();
        for i in 0..NUM_AI_WEIGHTS {
            write!(weight_str, "{:.2} ", weights.weights[i as usize])?;
        }
        write!(res, "\tWeights: {}\n", weight_str.trim())?;
    }
    Ok(res)
}

// Used in embedded CLI
fn run_api(weight_str: &str, print_eval: bool) -> Result<(), std::io::Error> {
    eprintln!("Hello, from rust-ai!");
    let mut game = Game::new();
    let ai = AI::new(&AIWeights::from_string(weight_str).unwrap());
    loop {
        let input = ai_api::api_read().unwrap();
        // Special case
        if let None = input.hold {
            ai_api::api_write(ai_api::APIOutput {
                moves: vec![APIMove::Hold],
                score: None,
            })
            .unwrap();
            continue;
        }

        let hold = Piece::from_i32(input.hold.unwrap()).unwrap();
        game.set_hold(hold);
        let current = Piece::from_i32(input.current).unwrap();
        let mut queue = vec![current];
        for num in input.queue {
            queue.push(Piece::from_i32(num).unwrap());
        }
        game.set_queue(queue);
        let mut matrix = [0; BOARD_HEIGHT as usize];
        for j in 0..BOARD_VISIBLE_HEIGHT {
            matrix[j as usize] = input.matrix[j as usize];
        }
        game.board.set_matrix(matrix);

        // Evaluate
        let eval = ai.evaluate(&mut game, 3);

        // Write moves
        let mut moves = Vec::new();
        if eval.drop.hold {
            moves.push(APIMove::Hold);
        }
        match eval.drop.rotation {
            0 => {}
            1 => moves.push(APIMove::RotateLeft),
            2 => moves.push(APIMove::Rotate180),
            3 => moves.push(APIMove::RotateRight),
            _ => panic!(),
        }
        for _ in 0..(eval.drop.shift.abs()) {
            if eval.drop.shift < 0 {
                moves.push(APIMove::ShiftLeft)
            } else {
                moves.push(APIMove::ShiftRight)
            }
        }
        moves.push(APIMove::HardDrop);
        ai_api::api_write(ai_api::APIOutput {
            moves,
            score: Some(eval.score as f64),
        })
        .unwrap();

        // Print evaluation
        if print_eval {
            eprintln!(
                "{}",
                write_state(&game, &eval.drop, Some(&eval), Some(&ai.weights), None).unwrap()
            );
        }
    }
}

fn test() {
    let mut weights = AIWeights::new();
    weights.weights = [1.0, 1.0, 1.0, 1.0, 1.0, 0.0, -10.0, -10.0, 0.0];
    let weights = weights.normalized();
    println!("{}", weights.to_string())
}
