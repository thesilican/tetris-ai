use mongodb::{bson::doc, sync::Client};
use pc_finder::*;
use serde::Deserialize;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct DbBoard {
    #[serde(rename = "_id")]
    id: PcBoard,
    assigned: bool,
    visited: bool,
    backlinks: Vec<PcBoard>,
    children: Vec<PcBoard>,
}

static EXIT: AtomicBool = AtomicBool::new(false);

// Take the result of gen and write backlinks
fn main() {
    // Set up mongodb connection
    let uri = std::env::args()
        .nth(1)
        .unwrap_or(String::from("mongodb://localhost:27017"));
    let client = Client::with_uri_str(uri).unwrap();
    let collection = client.database("pc-finder").collection::<DbBoard>("boards");

    // Capture Ctrl+C
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    // Main loop
    let start = Instant::now();
    let mut count = 0;
    let mut stopwatch = AvgStopwatch::new(100);
    while !EXIT.load(Ordering::Relaxed) {
        stopwatch.start();
        // Find an unassigned board and assign to self
        let board = collection
            .find_one_and_update(
                doc! { "assigned": false },
                doc! { "$set": { "assigned": true }},
                None,
            )
            .unwrap();
        if let None = board {
            println!("Finished creating backlinks!");
            break;
        }
        let board = board.unwrap();

        // Add backlinks
        let board_id = board.id.to_i64();
        let child_ids = board
            .children
            .iter()
            .map(|x| x.to_i64())
            .collect::<Vec<_>>();
        collection
            .update_one(
                doc! { "_id": { "$in": child_ids } },
                doc! { "$addToSet": { "backlinks": board_id } },
                None,
            )
            .unwrap();

        // Mark board as visited
        collection
            .update_one(
                doc! { "_id": board_id },
                doc! { "$set": { "visited": true } },
                None,
            )
            .unwrap();

        // Display info
        count += 1;
        stopwatch.stop();
        println!(
            "Board {}\n{}\nWriting {} backlinks\nCount: {}\nAverage Time: {:?}\nTotal Time: {:?}\n",
            board_id,
            board.id,
            board.children.len(),
            count,
            stopwatch.reading(),
            start.elapsed()
        );
    }
}
