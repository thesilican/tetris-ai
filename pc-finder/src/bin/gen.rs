use mongodb::bson::doc;
use mongodb::options::InsertManyOptions;
use mongodb::{sync::Client, IndexModel};
use pc_finder::model::*;
use pc_finder::util::AvgStopwatch;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DbBoard {
    #[serde(rename = "_id")]
    id: PcBoard,
    assigned: bool,
    visited: bool,
    depth: i32,
    backlinks: Vec<PcBoard>,
    children: Vec<PcBoard>,
}
impl Default for DbBoard {
    fn default() -> Self {
        DbBoard {
            id: PcBoard::new(),
            assigned: false,
            visited: false,
            depth: 0,
            backlinks: vec![],
            children: vec![],
        }
    }
}

static EXIT: AtomicBool = AtomicBool::new(false);

fn ignore_duplicate_keys(
    result: mongodb::error::Result<mongodb::results::InsertManyResult>,
) -> Result<(), String> {
    use mongodb::error::{BulkWriteFailure, ErrorKind};
    match result {
        Ok(_) => Ok(()),
        Err(err) => match &*err.kind {
            ErrorKind::BulkWrite(BulkWriteFailure {
                write_errors: Some(bulk_write_errors),
                write_concern_error: None,
                ..
            }) => {
                for error in bulk_write_errors {
                    if error.code != 11000 {
                        return Err(format!(
                            "encountered BulkWriteError that is not E11000: {:?}",
                            error
                        ));
                    }
                }
                Ok(())
            }
            _ => Err(format!("not a bulk write error: {:?}", err)),
        },
    }
}

// DFS to generate all possible boards
fn main() {
    // Set up mongodb connection
    let uri = std::env::args()
        .nth(1)
        .unwrap_or(String::from("mongodb://localhost:27017"));
    let client = Client::with_uri_str(uri).unwrap();
    let collection = client.database("pc-finder").collection::<DbBoard>("boards");

    // Create indices
    collection
        .create_index(
            IndexModel::builder()
                .keys(doc! { "assigned": 1i32 })
                .build(),
            None,
        )
        .unwrap();

    // Initialize database if empty
    if collection.count_documents(None, None).unwrap() == 0 {
        collection.insert_one(DbBoard::default(), None).unwrap();
    }

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
                doc! { "assigned": false, "depth": { "$lte": 3 } },
                doc! { "$set": { "assigned": true }},
                None,
            )
            .unwrap();
        if let None = board {
            println!("Finished finding PCs!");
            break;
        }
        let board = board.unwrap();
        let depth = board.depth;

        // DFS children
        let children = board.id.child_boards();

        // Insert children
        if children.len() > 0 {
            let db_children = children.iter().map(|&board| DbBoard {
                id: board,
                depth: depth + 1,
                ..DbBoard::default()
            });
            let result = collection.insert_many(
                db_children,
                InsertManyOptions::builder().ordered(false).build(),
            );
            // Allow E11000 (duplicate key error) errors,
            // exit on anything else
            ignore_duplicate_keys(result).unwrap();
        }

        // Mark board as visited
        collection
            .update_one(
                doc! { "_id": board.id.to_i64() },
                doc! {
                    "$set": {
                        "visited": true,
                        "children": children.iter().map(|&board| board.to_i64()).collect::<Vec<_>>()
                    }
                },
                None,
            )
            .unwrap();

        // Display Info
        count += 1;
        stopwatch.stop();
        println!(
            "Board {}\n{}\nDepth {}\nFound {} children\nCount: {}\nAverage Time: {:?}\nTotal Time: {:?}\n",
            board.id.to_u64(),
            board.id,
            depth,
            children.len(),
            count,
            stopwatch.reading(),
            start.elapsed()
        );
    }
}
