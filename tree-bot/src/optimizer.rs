use std::{
    io::Write,
    sync::atomic::{AtomicI32, Ordering},
};

use libtetris::{ActionInfo, Ai, Bag, Evaluation, Game};
use rand::{Rng, RngCore, SeedableRng};
use rand_distr::{Distribution, Normal, Uniform};
use rand_xorshift::XorShiftRng;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{Params, TreeAi, PARAMS_DIM};

const POPULATION_KEEP: usize = 100;
const POPULATION_NEW: usize = 100;
const POPULATION_COMBINE_KEEP: usize = 700;
const POPULATION_COMBINE_ALL: usize = 100;
const POPULATION_SIZE: usize =
    POPULATION_KEEP + POPULATION_NEW + POPULATION_COMBINE_KEEP + POPULATION_COMBINE_ALL;

pub struct Optimizer {
    epoch: i32,
    rng: XorShiftRng,
    population: Vec<Params>,
}

impl Optimizer {
    pub fn new(seed: u64) -> Self {
        Optimizer {
            epoch: 0,
            rng: XorShiftRng::seed_from_u64(seed),
            population: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.epoch = 0;
        for _ in 0..POPULATION_SIZE {
            let params = self.random_params();
            self.population.push(params);
        }
    }

    pub fn perform_epoch(&mut self) {
        assert_eq!(self.population.len(), POPULATION_SIZE);
        self.epoch += 1;

        // Rank the population
        let (best, avg) = self.rank_population();
        println!("Epoch {}: best={} avg={}", self.epoch, best, avg);
        println!("Best: {:?}", self.population[0]);

        // Keep population best
        let mut new_population = Vec::new();
        new_population.extend(self.population.iter().take(POPULATION_KEEP).cloned());

        // Take random samples
        for _ in 0..POPULATION_NEW {
            new_population.push(self.random_params());
        }

        // Combine best
        let uniform = Uniform::<f32>::new(0., 1.).unwrap();
        for _ in 0..POPULATION_COMBINE_KEEP {
            let idx1 = self.rng.random_range(0..POPULATION_KEEP);
            let idx2 = self.rng.random_range(0..POPULATION_KEEP);
            let vec1 = self.population[idx1].to_vec();
            let vec2 = self.population[idx2].to_vec();
            let weight = uniform.sample(&mut self.rng);
            let mut new_vec = [0.; PARAMS_DIM];
            for i in 0..PARAMS_DIM {
                new_vec[i] = vec1[i] * weight + vec2[i] * (1. - weight);
            }
            new_population.push(Params::from_vec(new_vec));
        }

        // Combine all
        for _ in 0..POPULATION_COMBINE_ALL {
            let idx1 = self.rng.random_range(0..(POPULATION_KEEP + POPULATION_NEW));
            let idx2 = self.rng.random_range(0..(POPULATION_KEEP + POPULATION_NEW));
            let vec1 = self.population[idx1].to_vec();
            let vec2 = self.population[idx2].to_vec();
            let weight = uniform.sample(&mut self.rng);
            let mut new_vec = [0.; PARAMS_DIM];
            for i in 0..PARAMS_DIM {
                new_vec[i] = vec1[i] * weight + vec2[i] * (1. - weight);
            }
            new_population.push(Params::from_vec(new_vec));
        }

        self.population = new_population
    }

    pub fn rank_population(&mut self) -> (f32, f32) {
        let count = AtomicI32::new(0);
        print!("Ranking 0/{POPULATION_SIZE}");

        let seed = self.rng.next_u64();
        let mut scored_population = self
            .population
            .par_iter()
            .map(|params| {
                let fitness = Self::compute_fitness(*params, seed);
                let val = count.fetch_add(1, Ordering::Relaxed) + 1;
                print!("\rRanking {val}/{POPULATION_SIZE}");
                std::io::stdout().flush().unwrap();
                (params.clone(), fitness)
            })
            .collect::<Vec<_>>();
        println!();

        scored_population.sort_by_key(|(_, fitness)| -*fitness);

        // Stats
        let best = scored_population[0].1 as f32;
        let avg = scored_population
            .iter()
            .map(|(_, fitness)| *fitness)
            .sum::<i32>() as f32
            / POPULATION_SIZE as f32;

        self.population = scored_population
            .into_iter()
            .map(|(params, _)| params)
            .collect();

        (best, avg)
    }

    pub fn random_params(&mut self) -> Params {
        let normal = Normal::new(0., 1.).unwrap();

        let mut vec = [0.0; PARAMS_DIM];
        for i in 0..PARAMS_DIM {
            vec[i] = normal.sample(&mut self.rng);
        }

        Params::from_vec(vec)
    }

    pub fn compute_fitness(params: Params, seed: u64) -> i32 {
        let mut lc1 = 0;
        let mut lc2 = 0;
        let mut lc3 = 0;
        let mut lc4 = 0;
        let mut ts1 = 0;
        let mut ts2 = 0;
        let mut ts3 = 0;

        let mut tree_ai = TreeAi::new(5, 4, params);
        let mut bag = Bag::new_rng7(seed);
        let mut game = Game::from_bag(&mut bag);
        'outer: for _ in 0..800 {
            let eval = tree_ai.evaluate(&game);
            let actions = match eval {
                Evaluation::Success { actions, .. } => actions,
                Evaluation::Fail { .. } => break,
            };
            for action in actions {
                let result = game.apply(action);
                match result {
                    ActionInfo::Success => {}
                    ActionInfo::Lock(lock_info) => {
                        if lock_info.top_out {
                            break;
                        }
                        match (lock_info.tspin, lock_info.lines_cleared) {
                            (true, 1) => ts1 += 1,
                            (true, 2) => ts2 += 1,
                            (true, 3) => ts3 += 1,
                            (false, 1) => lc1 += 1,
                            (false, 2) => lc2 += 1,
                            (false, 3) => lc3 += 1,
                            (false, 4) => lc4 += 1,
                            _ => {}
                        }
                    }
                    ActionInfo::Fail => break 'outer,
                }
            }
            game.refill_queue(&mut bag);
        }
        ts1 * 5 + ts2 * 10 + ts3 * 15 + lc1 * 0 + lc2 * 1 + lc3 * 2 + lc4 * 10
    }
}
