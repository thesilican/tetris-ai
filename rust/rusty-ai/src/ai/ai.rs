use crate::model::consts::BOARD_WIDTH;
use crate::model::game::Game;
use crate::model::game::GameDropOptions;
use crate::model::game::GameDropResult;
use crate::model::piece::Piece;
use byteorder::BigEndian;
use byteorder::{ReadBytesExt, WriteBytesExt};
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::io::Cursor;

pub const TARGET_HEIGHT: i32 = 0;

pub struct AI {
    pub weights: AIWeights,
}
impl AI {
    pub fn new(weights: &AIWeights) -> Self {
        AI {
            weights: weights.clone(),
        }
    }
    pub fn evaluate(&self, game: &mut Game, depth: i32) -> AIEvaluation {
        return self.evaluate_recursive(game, depth);
    }
    fn evaluate_recursive(&self, game: &mut Game, depth: i32) -> AIEvaluation {
        if game.queue_len <= 1 || depth == 0 {
            return AIEvaluation {
                score: self.board_score(&game),
                drop: GameDropOptions {
                    hold: false,
                    rotation: 0,
                    shift: 0,
                },
            };
        }

        let mut best_drop = GameDropOptions {
            hold: false,
            rotation: 0,
            shift: 0,
        };
        let mut best_score = f32::NEG_INFINITY;
        for i in 0..2 {
            let hold = i % 2 == 1;
            let piece = if hold {
                game.hold.clone()
            } else {
                game.get_curr_piece().unwrap().clone()
            };
            let rotation_bound = *Piece::info_rotation_bounds(&piece);
            for rotation in 0..rotation_bound {
                let (left_bound, right_bound) = Piece::info_shift_bounds(&piece, rotation);
                for shift in -left_bound..(right_bound + 1) {
                    let drop_res = game.drop(&GameDropOptions {
                        hold,
                        rotation,
                        shift,
                    });
                    if let Err(_) = drop_res {
                        continue;
                    }
                    let (drop_res, undo_info) = drop_res.unwrap();
                    let drop_score = self.drop_score(&drop_res);
                    let ai_res = self.evaluate_recursive(game, depth - 1);
                    let score = drop_score + ai_res.score;
                    if score > best_score {
                        best_score = score;
                        best_drop.hold = hold;
                        best_drop.rotation = rotation;
                        best_drop.shift = shift;
                    }
                    game.undo(&undo_info);
                }
            }
        }
        AIEvaluation {
            score: best_score,
            drop: best_drop,
        }
    }
    fn board_score(&self, game: &Game) -> f32 {
        let mut bumpiness = 0.0;
        for i in 1..BOARD_WIDTH {
            bumpiness += (game.board.height_map[i as usize]
                - game.board.height_map[(i - 1) as usize])
                .abs() as f32
        }
        let mut target_height = 0.0;
        for i in 0..BOARD_WIDTH {
            target_height += (game.board.height_map[i as usize] - TARGET_HEIGHT).abs() as f32
        }

        let clear_column = game.board.height_map[0] as f32;

        let holes = game.board.holes as f32;

        let weights = self.weights.weights;
        bumpiness * weights[5]
            + target_height * weights[6]
            + holes * weights[7]
            + clear_column * weights[8]
    }
    fn drop_score(&self, drop: &GameDropResult) -> f32 {
        (match drop.lines_cleared {
            0 => 0.0,
            1 => self.weights.weights[1],
            2 => self.weights.weights[2],
            3 => self.weights.weights[3],
            4 => self.weights.weights[4],
            _ => panic!("Unknown number of lines cleared"),
        }) + (match drop.perfect_clear {
            false => 0.0,
            true => self.weights.weights[0],
        })
    }
}

pub const NUM_AI_WEIGHTS: i32 = 9;

#[derive(Copy, Clone, Default, Debug)]
pub struct AIWeights {
    /// 0: Perfect Clear\
    /// 1: 1 line clear\
    /// 2: 2 line clear\
    /// 3: 3 line clear\
    /// 4: 4 line clear\
    /// 5: Bumpiness\
    /// 6: Target Height\
    /// 7: Holes\
    /// 8: Clear Column (left column height)
    pub weights: [f32; NUM_AI_WEIGHTS as usize],
}
impl AIWeights {
    pub fn new() -> Self {
        AIWeights {
            weights: [0.0; NUM_AI_WEIGHTS as usize],
        }
    }
    pub fn from_string(text: &str) -> Result<Self, ()> {
        let bytes = base64::decode(text).unwrap();
        let mut cursor = Cursor::new(bytes);
        let mut weights = [0.0; NUM_AI_WEIGHTS as usize];
        for i in 0..NUM_AI_WEIGHTS {
            weights[i as usize] = cursor.read_f32::<BigEndian>().unwrap();
        }
        Ok(AIWeights { weights })
    }
    pub fn normalized(&self) -> Self {
        let mut mag = 0.0;
        for i in 0..NUM_AI_WEIGHTS {
            mag += self.weights[i as usize].powf(2.0)
        }
        mag = mag.sqrt();
        // Prevent division by zero errors
        mag = if mag == 0.0 { 1.0 } else { mag };

        let mut weights = self.weights;
        for i in 0..NUM_AI_WEIGHTS {
            weights[i as usize] /= mag;
        }
        AIWeights { weights }
    }
    pub fn cross_over(&self, other: &Self, self_weight: f32, other_weight: f32) -> Self {
        let mut weights = [0.0; NUM_AI_WEIGHTS as usize];
        for i in 0..NUM_AI_WEIGHTS {
            weights[i as usize] =
                self.weights[i as usize] * self_weight + other.weights[i as usize] * other_weight
        }
        AIWeights { weights }.normalized()
    }
    pub fn mutate(&self, property: i32, amount: f32) -> Self {
        let mut weights = self.weights;
        weights[property as usize] = amount;
        AIWeights { weights }
    }
    pub fn to_string(&self) -> String {
        let mut bytes = Vec::new();
        for num in self.weights.iter() {
            bytes.write_f32::<BigEndian>(*num).unwrap();
        }
        base64::encode(bytes)
    }
}
impl Hash for AIWeights {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_string().hash(state)
    }
}
impl PartialEq for AIWeights {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for AIWeights {}
impl Display for AIWeights {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Clone)]
pub struct AIEvaluation {
    pub score: f32,
    pub drop: GameDropOptions,
}
