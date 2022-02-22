#![feature(control_flow_enum)]
use mongodb::bson::doc;
use mongodb::options::InsertManyOptions;
use mongodb::sync::Collection;
use mongodb::{sync::Client, IndexModel};
use pc_finder::gen::*;
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DbBoard {
    #[serde(rename = "_id")]
    id: PcBoard,
    assigned: bool,
    visited: bool,
    children: Vec<PcBoard>,
}
impl Default for DbBoard {
    fn default() -> Self {
        DbBoard {
            id: PcBoard::new(),
            assigned: false,
            visited: false,
            children: vec![],
        }
    }
}

static EXIT: AtomicBool = AtomicBool::new(false);

fn step(collection: &Collection<DbBoard>) -> Result<ControlFlow<(), ()>, mongodb::error::Error> {
    // Find an unassigned board and assign to self
    let board = collection.find_one_and_update(
        doc! { "assigned": false },
        doc! { "$set": { "assigned": true }},
        None,
    )?;
    if let None = board {
        println!("Finished finding PCs!");
        return Ok(ControlFlow::BREAK);
    }
    let board = board.unwrap();

    // DFS children
    let children = board.id.child_boards();
    println!(
        "Searching {}\n{}\nFound {} children\n",
        board.id.to_u64(),
        board.id,
        children.len()
    );

    // Mark board as visited
    collection.update_one(
        doc! { "_id": board.id.to_u64() as i64 },
        doc! {
            "$set": {
                "visited": true,
                "children": children.iter().map(|&board| board.to_u64() as i64).collect::<Vec<_>>()
            }
        },
        None,
    )?;

    // Insert children
    if children.len() > 0 {
        let db_children = children.iter().map(|&board| DbBoard {
            id: board,
            assigned: false,
            visited: false,
            children: vec![],
        });
        let result = collection.insert_many(
            db_children,
            InsertManyOptions::builder().ordered(false).build(),
        );
        // Allow E11000 (duplicate key error) errors,
        // exit on anything else
        match result {
            Ok(_) => (),
            Err(err) => match &*err.kind {
                mongodb::error::ErrorKind::BulkWrite(mongodb::error::BulkWriteFailure {
                    write_errors: Some(bulk_write_errors),
                    write_concern_error: None,
                    ..
                }) => {
                    for error in bulk_write_errors {
                        if error.code != 11000 {
                            return Err(err);
                        }
                    }
                }
                _ => return Err(err),
            },
        }
    }

    Ok(ControlFlow::CONTINUE)
}

// Generates a tree of results
fn main() -> Result<(), mongodb::error::Error> {
    // Set up mongodb connection
    let uri = std::env::var("MONGODB_URI").unwrap_or(String::from("mongodb://localhost:27017"));
    let client = Client::with_uri_str(uri)?;
    let collection = client.database("pc-finder").collection::<DbBoard>("boards");

    // Create indices
    collection.create_index(
        IndexModel::builder().keys(doc! { "visited": 1i32 }).build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "assigned": 1i32 })
            .build(),
        None,
    )?;

    // Initialize database if empty
    if collection.count_documents(None, None)? == 0 {
        collection.insert_one(DbBoard::default(), None)?;
    }

    // Capture Ctrl+C
    ctrlc::set_handler(|| EXIT.store(true, Ordering::Relaxed)).unwrap();

    // Main loop
    while !EXIT.load(Ordering::Relaxed) {
        match step(&collection)? {
            ControlFlow::Continue(_) => (),
            ControlFlow::Break(_) => break,
        }
    }

    Ok(())
}
