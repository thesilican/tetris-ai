use mongodb::{bson::doc, sync::Client};
use pc_finder::gen::PcBoard;
use serde::Deserialize;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Deserialize)]
struct DbBoard {
    #[serde(rename = "_id")]
    id: PcBoard,
    assigned: bool,
    visited: bool,
    children: Vec<PcBoard>,
    backlinks: Vec<PcBoard>,
}

static EXIT: AtomicBool = AtomicBool::new(false);

// Take the result of gen, and prune it down to only a tree of elements
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up mongodb connection
    let uri = std::env::var("MONGODB_URI").unwrap_or(String::from("mongodb://localhost:27017"));
    let client = Client::with_uri_str(uri)?;
    let collection = client.database("pc-finder").collection::<DbBoard>("boards");

    // Capture Ctrl+C
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    // Main loop
    while !EXIT.load(Ordering::Relaxed) {
        let board = collection.find_one_and_update(
            doc! { "assigned": false },
            doc! { "$set": { "assigned": true }},
            None,
        )?;
        if let None = board {
            println!("Finished creating backlinks!");
            break;
        }
        let board = board.unwrap();

        // Add backlinks
        let board_id = board.id.to_u64() as i64;
        let child_ids = board
            .children
            .iter()
            .map(|x| x.to_u64() as i64)
            .collect::<Vec<_>>();
        collection.update_one(
            doc! { "_id": { "$in": child_ids } },
            doc! { "$addToSet": { "backlinks": board_id } },
            None,
        )?;
        println!("Board {} with {} backlinks", board_id, board.children.len());
        collection.update_one(
            doc! { "_id": board_id },
            doc! { "$set": { "visited": true } },
            None,
        )?;
    }

    Ok(())
}
