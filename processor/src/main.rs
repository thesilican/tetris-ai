use std::fs::OpenOptions;

use processor::{FrameCollection, Replay, TestCase};
use rand::{
    prelude::{SliceRandom, StdRng},
    SeedableRng,
};

fn save_test_cases(name: &str, cases: &[TestCase]) {
    for (i, case) in cases.iter().enumerate() {
        let filename = format!("data/ml/{0}/{0}-{1}.json", name, i);
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(filename)
            .unwrap();
        serde_json::to_writer(file, case).unwrap();
    }
}

fn main() {
    let frames = FrameCollection::load();
    let replays = frames
        .iter()
        .map(Replay::from_frame_collection)
        .collect::<Vec<_>>();
    let mut rng = StdRng::seed_from_u64(1234);
    let (mut train, mut test) = replays
        .iter()
        .map(|x| TestCase::from_replay(&mut rng, x))
        .fold(Vec::new(), |mut a, v| {
            a.extend(v);
            a
        })
        .into_iter()
        .enumerate()
        .fold((Vec::new(), Vec::new()), |(mut train, mut test), (i, v)| {
            if i % 7 == 0 {
                test.push(v);
            } else {
                train.push(v);
            }
            (train, test)
        });
    train.shuffle(&mut rng);
    test.shuffle(&mut rng);
    println!(
        "Generated {} training, {} testing cases",
        train.len(),
        test.len()
    );

    save_test_cases("train", &train);
    save_test_cases("test", &test);
    println!("Finished generating training data");
}
