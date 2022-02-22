use mongodb::{
    bson::doc,
    sync::{Client, Collection},
};
use pc_finder::gen::PcBoard;
use serde::Deserialize;
use std::{
    ops::ControlFlow,
    sync::atomic::{AtomicBool, Ordering},
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

fn step(collection: &Collection<DbBoard>) -> ControlFlow<(), ()> {
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
        return ControlFlow::Break(());
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
    println!("Board {} with {} backlinks", board_id, board.children.len());

    // Mark board as visited
    collection
        .update_one(
            doc! { "_id": board_id },
            doc! { "$set": { "visited": true } },
            None,
        )
        .unwrap();

    ControlFlow::Continue(())
}

// Take the result of gen, and prune it down to only a tree of elements
fn main() {
    // Set up mongodb connection
    let uri = std::env::args()
        .next()
        .unwrap_or(String::from("mongodb://localhost:27017"));
    let client = Client::with_uri_str(uri).unwrap();
    let collection = client.database("pc-finder").collection::<DbBoard>("boards");

    // Capture Ctrl+C
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    // Main loop
    while !EXIT.load(Ordering::Relaxed) {
        match step(&collection) {
            ControlFlow::Continue(_) => (),
            ControlFlow::Break(_) => break,
        }
    }
}
