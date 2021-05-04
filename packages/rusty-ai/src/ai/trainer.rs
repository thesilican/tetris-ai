use crate::ai::ai::AIWeights;
use crate::ai::ai::AI;
use crate::ai::ai::NUM_AI_WEIGHTS;
use crate::model::game::GameRNGGenerator;
use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::Normal;
use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::RwLock;
use std::thread::JoinHandle;

pub const FITNESS_FILE_PATH: &str = "fitness-cache.txt";

pub const AI_DEPTH: i32 = 3;
pub const AI_GAME_ROUNDS: i32 = 500;
pub const AI_GARBAGE_FREQ: i32 = 8;

pub const ROUNDS: i32 = 100;
pub const POPULATION: i32 = 2000;
pub const SELECTION_AMOUNT: i32 = 50;
pub const OFFSPRING_AMOUNT: i32 = 500;
pub const MUTATION_CHANCE: f32 = 0.5;
pub const MUTATION_AMOUNT: f32 = 0.2;

pub const SCORE_WEIGHT: i32 = 1;
pub const TOP_OUT_WEIGHT: i32 = -2;
pub const HOLE_WEIGHT: i32 = -1;

pub const THREAD_COUNT: i32 = 30;

pub struct AITrainer {
    rng: StdRng,
    fitness_map: HashMap<(AIWeights, i32), i32>,
    should_exit: Arc<AtomicBool>,
}
impl AITrainer {
    pub fn new() -> Self {
        AITrainer {
            should_exit: Arc::new(AtomicBool::new(false)),
            rng: StdRng::seed_from_u64(0),
            fitness_map: HashMap::new(),
        }
    }
    pub fn start(&mut self) {
        // Trap Ctrl-C
        let local_should_exit = self.should_exit.clone();
        ctrlc::set_handler(move || {
            println!("Gracefully exiting...");
            local_should_exit.store(true, Ordering::Relaxed);
        })
        .expect("Error setting Ctrl-C handler");

        // Load fitness map
        self.fitness_map.clear();
        self.rng = StdRng::seed_from_u64(0);
        if let Err(_) = self.load_fitness_file() {
            println!("Error loading {}", FITNESS_FILE_PATH);
        }

        // Initial population
        let mut population = [AIWeights::new(); POPULATION as usize];
        for i in 0..POPULATION {
            population[i as usize] = self.get_random_weights();
        }

        for round in 1..ROUNDS + 1 {
            println!("Training round {}", round);
            self.train_fitness_threads(&population, round);
            if self.should_exit.load(Ordering::Relaxed) {
                return;
            }

            // Print training info
            let mut min_fitness = i32::MAX;
            let mut min_weight = None;
            let mut max_fitness = i32::MIN;
            let mut max_weight = None;
            let mut avg_fitness = 0;
            for i in 0..POPULATION {
                let weight = population[i as usize];
                let fitness = self.fitness_map.get(&(weight, round)).unwrap();
                if *fitness < min_fitness {
                    min_weight = Some(weight);
                    min_fitness = *fitness;
                }
                if *fitness > max_fitness {
                    max_weight = Some(weight);
                    max_fitness = *fitness;
                }
                avg_fitness += fitness;
            }
            avg_fitness /= POPULATION;
            println!("Training round {} completed:", round);
            println!("\tMin: {} {}", min_fitness, min_weight.unwrap());
            println!("\tMax: {} {}", max_fitness, max_weight.unwrap());
            println!("\tAvg: {}", avg_fitness);

            println!("Generating offspring round {}", round);
            let mut offspring_list = Vec::<AIWeights>::new();
            for _ in 0..OFFSPRING_AMOUNT {
                // Select from population
                shuffle(&mut population, &mut self.rng);
                let mut population_selection: [AIWeights; SELECTION_AMOUNT as usize] =
                    population[0..SELECTION_AMOUNT as usize].try_into().unwrap();
                population_selection.sort_by(|a, b| {
                    let fitness_a = self.fitness_map.get(&(*a, round)).unwrap();
                    let fitness_b = self.fitness_map.get(&(*b, round)).unwrap();
                    fitness_b.partial_cmp(fitness_a).unwrap()
                });
                let top_1 = population_selection[0];
                let top_2 = population_selection[1];
                let fitness_1 = *self.fitness_map.get(&(top_1, round)).unwrap();
                let fitness_2 = *self.fitness_map.get(&(top_2, round)).unwrap();
                let mut offspring = top_1.cross_over(&top_2, fitness_1 as f32, fitness_2 as f32);
                println!(
                    "\tCrossbreeding {} {}\n\t          and {} {}",
                    fitness_1, top_1, fitness_2, top_2
                );
                println!("\t\tChild: {}", offspring);

                // Mutate offspring
                loop {
                    let p = Uniform::new(0.0, 1.0).sample(&mut self.rng);
                    // Give many opportunities to mutate
                    if p < MUTATION_CHANCE {
                        let amount = Normal::new(0.0, MUTATION_AMOUNT)
                            .unwrap()
                            .sample(&mut self.rng);
                        let property = Uniform::new(0, NUM_AI_WEIGHTS).sample(&mut self.rng);
                        offspring = offspring.mutate(property, amount);
                        println!("\t\tMutating: {} {} {}", property, amount, offspring);
                    } else {
                        break;
                    }
                }
                offspring_list.push(offspring);
            }
            // Find bottom N indexes
            population.sort_by(|a, b| {
                let fitness_a = self.fitness_map.get(&(*a, round)).unwrap();
                let fitness_b = self.fitness_map.get(&(*b, round)).unwrap();
                fitness_a.partial_cmp(fitness_b).unwrap()
            });
            // Replace with offspring
            for i in 0..OFFSPRING_AMOUNT {
                let weights = population[i as usize];
                let fitness = self.fitness_map.get(&(weights, round)).unwrap();
                let offspring = offspring_list[i as usize];
                println!(
                    "\tReplacing {} {}\n\t     with {}",
                    fitness, weights, offspring
                );
                population[i as usize] = offspring;
            }
        }
    }
    fn train_fitness_threads(&mut self, population: &[AIWeights; POPULATION as usize], round: i32) {
        // Calculate populations that are cached
        let mut cached = [None; POPULATION as usize];
        for (i, weights) in population.iter().enumerate() {
            if let Some(fitness) = self.fitness_map.get(&(*weights, round)) {
                cached[i] = Some(*fitness);
            }
        }

        let cached = Arc::new(RwLock::new(cached));
        let index = Arc::new(AtomicUsize::new(0));
        let population = Arc::new(RwLock::new(population.clone()));
        let mut threads = Vec::<JoinHandle<HashMap<(AIWeights, i32), i32>>>::new();
        for _ in 0..THREAD_COUNT {
            let cached = Arc::clone(&cached);
            let index = Arc::clone(&index);
            let population = Arc::clone(&population);
            let should_exit = Arc::clone(&self.should_exit);
            // Start worker threads
            let thread = std::thread::spawn(move || {
                let population = &*population.read().unwrap();
                let cached = &*cached.read().unwrap();
                let mut res = HashMap::<(AIWeights, i32), i32>::new();
                loop {
                    if should_exit.load(Ordering::Relaxed) {
                        return res;
                    }
                    let my_index = index.fetch_add(1, Ordering::Relaxed);
                    if my_index >= POPULATION as usize {
                        return res;
                    }
                    let weights = &population[my_index];
                    if let Some(fitness) = cached[my_index as usize] {
                        println!(
                            "\tWeights [{},{}] cached: {} {}",
                            round, my_index, fitness, weights
                        );
                        continue;
                    }
                    let fitness = AITrainer::get_fitness(weights, round as u64, &should_exit);
                    match fitness {
                        Ok(fitness) => {
                            println!(
                                "\tWeights [{},{}] finished: {} {}",
                                round, my_index, fitness, weights
                            );
                            res.insert((weights.clone(), round), fitness);
                        }
                        Err(_) => {
                            println!("\tWeights [{},{}] aborted", round, my_index);
                            return res;
                        }
                    };
                }
            });
            threads.push(thread);
        }
        let mut i = 0;
        for thread in threads {
            let res = thread.join().unwrap();
            for (key, val) in res {
                self.fitness_map.insert(key, val);
            }
            println!("\tThread {} joined", i);
            i += 1;
        }
        self.save_fitness_file();
    }
    fn get_fitness(
        weight: &AIWeights,
        seed: u64,
        should_exit: &Arc<AtomicBool>,
    ) -> Result<i32, ()> {
        let mut ai = AI::new(weight, false);
        let mut rng = GameRNGGenerator::new(Some(seed));
        ai.game.extend_queue(rng.gen_7bag());
        for tick in 0..AI_GAME_ROUNDS {
            if tick % 10 == 0 && should_exit.load(Ordering::Relaxed) {
                return Err(());
            }
            let eval = ai.evaluate_recursive(AI_DEPTH);
            let drop_res = ai.game.drop(&eval.drop);
            if let Err(_) = drop_res {
                // Early death
                return Ok(ai.game.score * SCORE_WEIGHT
                    + ai.game.board.holes * HOLE_WEIGHT
                    + (AI_GAME_ROUNDS - tick) * TOP_OUT_WEIGHT);
            }
            if tick % AI_GARBAGE_FREQ == 0 {
                ai.game.board.add_garbage_line(rng.gen_garbage_line());
            }
            if ai.game.queue_len < 7 {
                ai.game.extend_queue(rng.gen_7bag());
            }
        }
        Ok(ai.game.score * SCORE_WEIGHT + ai.game.board.holes * HOLE_WEIGHT)
    }
    fn get_random_weights(&mut self) -> AIWeights {
        let mut weights = AIWeights::new();
        let distr = Normal::new(0.0, 1.0).unwrap();
        for i in 0..NUM_AI_WEIGHTS {
            weights.weights[i as usize] = distr.sample(&mut self.rng);
        }
        weights.normalized()
    }
    fn load_fitness_file(&mut self) -> Result<(), std::io::Error> {
        println!("Loading {}...", FITNESS_FILE_PATH);
        let bytes = std::fs::read(FITNESS_FILE_PATH)?;
        let text = String::from_utf8(bytes).expect("Error converting bytes to UTF-8");
        self.fitness_map.clear();
        for line in text.trim().split('\n') {
            let mut fitness: Option<i32> = None;
            let mut round: Option<i32> = None;
            let mut ai_weights: Option<AIWeights> = None;
            for (i, part) in line.split(' ').enumerate() {
                if i == 0 {
                    round = Some(part.parse::<i32>().unwrap());
                } else if i == 1 {
                    fitness = Some(part.parse::<i32>().unwrap());
                } else if i == 2 {
                    ai_weights = Some(AIWeights::from_string(part).unwrap());
                }
            }
            self.fitness_map
                .insert((ai_weights.unwrap(), round.unwrap()), fitness.unwrap());
        }
        Ok(())
    }
    fn save_fitness_file(&self) {
        println!("Saving {}...", FITNESS_FILE_PATH);
        let mut text = String::new();
        for ((ai_weights, round), fitness) in self.fitness_map.iter() {
            let mut line = String::new();
            line.push_str(&*round.to_string());
            line.push(' ');
            line.push_str(&*fitness.to_string());
            line.push(' ');
            line.push_str(&ai_weights.to_string());
            line.push('\n');
            text.push_str(&line);
        }
        std::fs::write(FITNESS_FILE_PATH, text.trim().as_bytes()).expect("Error writing file");
    }
}

fn shuffle<T: Clone>(arr: &mut [T; POPULATION as usize], rng: &mut StdRng) {
    for i in (1..POPULATION).rev() {
        let distribution = Uniform::new(0, i + 1);
        let j = distribution.sample(rng);
        let tmp = arr[i as usize].clone();
        arr[i as usize] = arr[j as usize].clone();
        arr[j as usize] = tmp;
    }
}
