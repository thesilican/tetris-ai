use common::misc::ThreadPool;
use processor::{FrameCollection, Replay, TestCase};
use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};
use std::fs::OpenOptions;

fn save_test_cases(name: &str, cases: &[TestCase]) {
    // // Save test cases individually
    // for (i, case) in cases.iter().enumerate() {
    //     let filename = format!("data/ml/{0}/{0}-{1}.json", name, i);
    //     let file = OpenOptions::new()
    //         .write(true)
    //         .create(true)
    //         .open(filename)
    //         .unwrap();
    //     serde_json::to_writer(file, case).unwrap();
    // }

    // Save test cases into 1 file
    let filename = format!("data/ml/{}.json", name);
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();
    serde_json::to_writer(file, cases).unwrap();
}

fn main() {
    const SEED: u64 = 1234;

    let frames = FrameCollection::load();
    let replays = frames
        .iter()
        .map(Replay::from_frame_collection)
        .collect::<Vec<_>>();

    // Generate test cases using thread pool (for parallelization)
    let mut thread_pool = ThreadPool::new(20);
    let jobs = replays
        .into_iter()
        .enumerate()
        .map(|(i, replay)| {
            move || {
                let mut rng = StdRng::seed_from_u64(SEED + i as u64);
                TestCase::from_replay(&mut rng, &replay)
            }
        })
        .collect::<Vec<_>>();
    let test_cases = thread_pool.run(jobs);
    // Flatten
    let mut test_cases = test_cases.into_iter().fold(vec![], |mut a, v| {
        a.extend(v);
        a
    });
    let mut rng = StdRng::seed_from_u64(SEED);
    test_cases.shuffle(&mut rng);

    // Take only the first 70,000 test cases
    let iter = test_cases.into_iter().take(70_000).enumerate();
    let mut train = Vec::new();
    let mut test = Vec::new();
    for (i, case) in iter {
        if i % 7 == 0 {
            test.push(case)
        } else {
            train.push(case);
        }
    }

    println!(
        "Generated {} training, {} testing cases",
        train.len(),
        test.len()
    );

    save_test_cases("train", &train);
    save_test_cases("test", &test);
    println!("Finished saving training data");
}
