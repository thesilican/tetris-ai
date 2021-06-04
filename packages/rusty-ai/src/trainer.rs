use crate::ai::RustyAI;
use crate::ai_weights::AIWeights;
use crate::ai_weights::NUM_AI_WEIGHTS;
use crate::threading::ThreadPool;
use common::api::ai::{TetrisAI, TetrisAIRes};
use common::misc::GenericErr;
use common::model::game::GameDropRes;
use common::model::game::{Game, GameMove, GameMoveRes};
use common::model::piece::PieceType;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::Distribution;
use rand_distr::Normal;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;
use std::sync::atomic::{self, AtomicBool};
use std::sync::Arc;

pub const FITNESS_FILE_PATH: &str = "fitness-cache.txt";

pub const AI_GAME_ROUNDS: i32 = 500;
pub const AI_GARBAGE_FREQ: i32 = 8;
pub const AI_TOP_OUT_WEIGHT: i32 = -2;

pub const ROUNDS: i32 = 100;
pub const POPULATION: i32 = 2000;
pub const SELECTION_AMOUNT: usize = 100;
pub const OFFSPRING_AMOUNT: i32 = 1500;
pub const MUTATION_CHANCE: f32 = 0.6;
pub const MUTATION_AMOUNT: f32 = 0.1;
pub const MIN_FITNESS: i32 = AI_TOP_OUT_WEIGHT * AI_GAME_ROUNDS;

pub const THREAD_COUNT: usize = 8;

fn shuffle<T>(arr: &mut [T], rng: &mut StdRng) {
    for i in (1..arr.len()).rev() {
        let j = Uniform::from(0..=i).sample(rng);
        arr.swap(i as usize, j as usize);
    }
}

pub struct FitnessMap(HashMap<(AIWeights, i32), i32>);
impl FitnessMap {
    pub fn new() -> Self {
        FitnessMap(HashMap::new())
    }
}
impl Display for FitnessMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for ((weight, round), fitness) in self.0.iter() {
            writeln!(f, "{} {} {}", round, fitness, weight)?;
        }
        Ok(())
    }
}
impl FromStr for FitnessMap {
    type Err = GenericErr;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let splits = text.trim().split('\n');
        let mut map = HashMap::new();
        for (i, row) in splits.enumerate() {
            let mut item = row.trim().split_ascii_whitespace();
            let round = match item.next() {
                Some(text) => text.parse()?,
                None => return Err(format!("Row {} missing round: '{}'", i, row).into()),
            };
            let fitness = match item.next() {
                Some(text) => text.parse()?,
                None => return Err(format!("Row {} missing fitness: '{}'", i, row).into()),
            };
            let weight = match item.next() {
                Some(text) => text.parse()?,
                None => return Err(format!("Row {} missing weight: '{}'", i, row).into()),
            };
            map.insert((weight, round), fitness);
        }
        Ok(FitnessMap(map))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct PopWeight(pub usize, pub AIWeights);
impl PartialOrd for PopWeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}
impl Ord for PopWeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

#[derive(Debug, Clone)]
pub struct Population {
    round: i32,
    weights: Vec<PopWeight>,
}
impl Population {
    fn generate_new(rng: &mut StdRng) -> Self {
        let mut weights = Vec::new();
        for i in 0..POPULATION {
            weights.push(PopWeight(i as usize, AIWeights::new_random(rng)));
        }
        Population { round: 1, weights }
    }
    fn generate_offspring(&self, fitness_map: &FitnessMap, rng: &mut StdRng) -> Self {
        let round = self.round;
        let fitness_cmp = |a: &PopWeight, b: &PopWeight| {
            let fitness_a = fitness_map.0.get(&(a.1, round)).unwrap();
            let fitness_b = fitness_map.0.get(&(b.1, round)).unwrap();
            fitness_a.cmp(fitness_b)
        };
        // Local mutable copy
        let mut weights = self.weights.clone();
        let mut offsprings = Vec::new();
        for i in 0..OFFSPRING_AMOUNT {
            shuffle(&mut weights, rng);
            let offspring_pool = &mut weights[0..SELECTION_AMOUNT];
            offspring_pool.sort_by(fitness_cmp);
            let PopWeight(index_1, top_1) = offspring_pool[offspring_pool.len() - 1];
            let PopWeight(index_2, top_2) = offspring_pool[offspring_pool.len() - 2];
            let fitness_1 = *fitness_map.0.get(&(top_1, round)).unwrap();
            let fitness_2 = *fitness_map.0.get(&(top_2, round)).unwrap();
            let weight_1 = (fitness_1 - MIN_FITNESS) as f32;
            let weight_2 = (fitness_2 - MIN_FITNESS) as f32;
            let mut offspring = top_1.cross_over(&top_2, weight_1, weight_2);
            println!(
                "\tCrossbreeding ({}, {}) {} and ({}, {}) {} => offspring [{}, {}]",
                round,
                index_1,
                fitness_1,
                round,
                index_2,
                fitness_2,
                round + 1,
                i,
            );
            // Mutate offspring
            loop {
                let p = Uniform::new(0.0, 1.0).sample(rng);
                // Give many opportunities to mutate
                if p < MUTATION_CHANCE {
                    let amount = Normal::new(0.0, MUTATION_AMOUNT).unwrap().sample(rng);
                    let property = Uniform::new(0, NUM_AI_WEIGHTS).sample(rng);
                    offspring = offspring.mutate(property, amount);
                    println!("\t\tMutating: {} {}", property, amount);
                } else {
                    break;
                }
            }
            offsprings.push((i, offspring));
        }
        // Find botom N indexes
        weights.sort_by(fitness_cmp);
        // Replace with offspring
        for (i, offspring) in offsprings {
            let PopWeight(index, weight) = weights[i as usize];
            let fitness = fitness_map.0.get(&(weight, round)).unwrap();
            println!(
                "\tReplacing ({}, {}) {} with offspring [{}, {}]",
                round,
                index,
                fitness,
                round + 1,
                i
            );
            weights[i as usize].1 = offspring;
        }
        weights.sort();
        Population {
            weights,
            round: round + 1,
        }
    }
}
impl Display for Population {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let round = self.round;
        let population_str = self
            .weights
            .iter()
            .map(|PopWeight(i, weight)| format!("\t({}, {}) {}", round, i, weight))
            .reduce(|mut acc, x| {
                acc.push('\n');
                acc.push_str(&x);
                acc
            })
            .unwrap();
        write!(f, "Round {} population:\n{}", round, population_str)?;
        Ok(())
    }
}

enum FitnessRes {
    Ok(i32),
    Err(String),
    Cached,
    Aborted,
}

pub struct AITrainer {
    rng: StdRng,
    fitness_map: FitnessMap,
    should_exit: Arc<AtomicBool>,
    thread_pool: ThreadPool<FitnessRes>,
}
impl AITrainer {
    pub fn new() -> Self {
        AITrainer {
            rng: StdRng::seed_from_u64(0),
            fitness_map: FitnessMap::new(),
            should_exit: Arc::new(AtomicBool::new(false)),
            thread_pool: ThreadPool::new(THREAD_COUNT),
        }
    }

    fn load_fitness_file(&mut self) -> Result<(), GenericErr> {
        println!("Loading {}...", FITNESS_FILE_PATH);
        let bytes = std::fs::read(FITNESS_FILE_PATH)?;
        self.fitness_map = String::from_utf8(bytes)?.parse()?;
        Ok(())
    }
    fn save_fitness_file(&self) -> Result<(), GenericErr> {
        println!("Saving {}...", FITNESS_FILE_PATH);
        std::fs::write(
            FITNESS_FILE_PATH,
            format!("{}", self.fitness_map).as_bytes(),
        )?;
        Ok(())
    }

    fn get_drop_res_score(drop_res: &GameDropRes, game: &Game) -> Result<i32, ()> {
        let drop_score = match drop_res.lines_cleared {
            0 => 0,
            1 => 0,
            2 => 1,
            3 => 2,
            4 => 4,
            _ => return Err(()),
        };
        let perfect_clear_score = if game.board.matrix[0] == 0 { 12 } else { 0 };
        Ok(drop_score + perfect_clear_score)
    }

    fn compute_weight_fitness(
        weights: AIWeights,
        seed: u64,
        should_exit: &Arc<AtomicBool>,
    ) -> FitnessRes {
        let mut ai = RustyAI::new(&weights, 0);
        let mut game = Game::new();
        let mut rng = StdRng::seed_from_u64(seed);
        let mut gen_bag = || {
            let mut bag = PieceType::iter_types().collect::<Vec<_>>();
            shuffle(&mut bag, &mut rng);
            bag
        };
        game.extend_queue(&gen_bag());
        game.make_move(GameMove::Hold);
        let mut score = 0;
        for tick in 0..AI_GAME_ROUNDS {
            // Check if should exit every now and then
            if tick % 10 == 0 && should_exit.load(atomic::Ordering::Relaxed) {
                return FitnessRes::Aborted;
            }
            let mut moves = match ai.api_evaluate(&game) {
                TetrisAIRes::Success { moves, .. } => moves,
                TetrisAIRes::Fail { .. } => {
                    // Just hard drop, probably a top out anyways
                    vec![GameMove::HardDrop]
                }
            };
            let last_move = match moves.pop() {
                Some(s) => s,
                None => return FitnessRes::Err("Moves length is 0".into()),
            };
            for game_move in moves {
                game.make_move(game_move);
            }
            let drop_res = match game.make_move(last_move) {
                GameMoveRes::SuccessDrop(drop_res) => drop_res,
                _ => return FitnessRes::Err("Last move was not hard drop".into()),
            };
            // Check score stuff
            score += match Self::get_drop_res_score(&drop_res, &game) {
                Ok(score) => score,
                Err(_) => return FitnessRes::Err("Error getting drop score".into()),
            };
            if drop_res.top_out {
                // Early top out
                let rounds_left = AI_GAME_ROUNDS - tick;
                score += AI_TOP_OUT_WEIGHT * rounds_left;
                return FitnessRes::Ok(score);
            }
            if game.queue_pieces.len() < 7 {
                game.extend_queue(&gen_bag());
            }
        }
        FitnessRes::Ok(score)
    }
    fn compute_population_fitness(
        &mut self,
        round: i32,
        population: &Population,
    ) -> Vec<FitnessRes> {
        let mut jobs = Vec::new();
        for PopWeight(i, weight) in population.weights.iter() {
            let i = *i;
            let weight = *weight;
            let cached = self.fitness_map.0.contains_key(&(weight, round));
            let should_exit = self.should_exit.clone();
            jobs.push(move || {
                if cached {
                    println!("\t({}, {}) cached", round, i);
                    return FitnessRes::Cached;
                }
                let res = AITrainer::compute_weight_fitness(weight, round as u64, &should_exit);
                let status = match &res {
                    FitnessRes::Ok(fitness) => {
                        format!("finished (fitness {})", fitness)
                    }
                    FitnessRes::Aborted => {
                        format!("aborted")
                    }
                    FitnessRes::Err(err) => {
                        format!("encountered error ({})", err)
                    }
                    // Should never happen
                    FitnessRes::Cached => unreachable!(),
                };
                println!("\t({}, {}) {}", round, i, status);
                res
            });
        }
        self.thread_pool.run_jobs(jobs)
    }

    pub fn start(&mut self) {
        // Trap Ctrl-C
        let local_should_exit = self.should_exit.clone();
        ctrlc::set_handler(move || {
            println!("Gracefully exiting...");
            local_should_exit.store(true, atomic::Ordering::Relaxed);
        })
        .unwrap();
        // Load files
        self.rng = StdRng::seed_from_u64(0);
        if let Err(_) = self.load_fitness_file() {
            println!("Error loading {}", FITNESS_FILE_PATH);
        }

        // Initial Population
        let mut population = Population::generate_new(&mut self.rng);
        // Hard code a relatively good 'seed' state
        let mut seed_weight = AIWeights::new();
        seed_weight.values = [
            1.0, // PC
            1.0, // 1 Line
            1.0, // 2 Line
            1.0, // 3 Line
            1.0, // 4 Line
            -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Holes
            -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, // Height
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, // Height Delta
        ];
        population.weights[0].1 = seed_weight;

        for round in 1..=ROUNDS {
            // Train
            println!("{}\nTraining round {}", population, round);
            let fitness_results = self.compute_population_fitness(round, &population);
            assert_eq!(fitness_results.len(), population.weights.len());
            let mut exit = false;
            for (result, PopWeight(_, weight)) in
                fitness_results.iter().zip(population.weights.iter())
            {
                match result {
                    FitnessRes::Ok(fitness) => {
                        self.fitness_map.0.insert((*weight, round), *fitness);
                    }
                    FitnessRes::Cached => {}
                    FitnessRes::Err(_) | FitnessRes::Aborted => {
                        exit = true;
                    }
                }
            }
            self.save_fitness_file().unwrap();
            if exit {
                println!("Exiting training program...");
                return;
            };

            // Print training information
            let mut min = (i32::MAX, None);
            let mut max = (i32::MIN, None);
            let mut avg_fitness = 0;
            for PopWeight(_, weight) in &population.weights {
                let fitness = self.fitness_map.0.get(&(*weight, round)).unwrap();
                if *fitness < min.0 {
                    min = (*fitness, Some(weight));
                }
                if *fitness > max.0 {
                    max = (*fitness, Some(weight));
                }
                avg_fitness += fitness;
            }
            avg_fitness /= POPULATION;
            println!("Training round {} completed:", round);
            println!("\tMin: {} {}", min.0, min.1.unwrap());
            println!("\tMax: {} {}", max.0, max.1.unwrap());
            println!("\tAvg: {}", avg_fitness);
            println!("Generating offspring for round {}", round);
            // Generate offspring
            population = population.generate_offspring(&self.fitness_map, &mut self.rng);
        }
        todo!()
    }
}
