#![feature(control_flow_enum)]
use mongodb::bson::doc;
use mongodb::options::InsertManyOptions;
use mongodb::sync::Collection;
use mongodb::{options::IndexOptions, sync::Client, IndexModel};
use pc_finder::*;
use serde::{Deserialize, Serialize};
use std::ops::ControlFlow;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DbBoard {
    board: PcBoard,
    visited: bool,
    children: Vec<PcBoard>,
}
impl Default for DbBoard {
    fn default() -> Self {
        DbBoard {
            board: PcBoard::new(),
            visited: false,
            children: vec![],
        }
    }
}

static EXIT: AtomicBool = AtomicBool::new(false);

fn step(collection: &Collection<DbBoard>) -> Result<ControlFlow<(), ()>, mongodb::error::Error> {
    // Find an unvisited board
    let board = collection.find_one(doc! { "visited": false }, None)?;
    if let None = board {
        println!("Finished finding PCs!");
        return Ok(ControlFlow::BREAK);
    }
    let board = board.unwrap();

    // DFS children
    let children = board.board.child_boards();
    println!(
        "Searching {}\n{}\nFound {} children\n",
        board.board.to_u64(),
        board.board,
        children.len()
    );

    // Insert children
    if children.len() > 0 {
        let db_children = children.iter().map(|&board| DbBoard {
            board,
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

    // Mark board as visited
    collection.update_one(
        doc! { "board": board.board.to_u64() as i64 },
        doc! {
            "$set": {
                "visited": true,
                "children": children.iter().map(|&board| board.to_u64() as i64).collect::<Vec<_>>()
            }
        },
        None,
    )?;
    Ok(ControlFlow::CONTINUE)
}

fn main() -> Result<(), mongodb::error::Error> {
    // Set up mongodb connection
    let uri = std::env::var("MONGODB_URI").unwrap_or(String::from("mongodb://localhost:27017"));
    let client = Client::with_uri_str(uri)?;
    let collection = client.database("pc-finder").collection::<DbBoard>("boards");

    // Ensure that the collection has a unique "board" index and visited index
    collection.create_index(
        IndexModel::builder()
            .keys(doc! { "board": 1i32 })
            .options(IndexOptions::builder().unique(true).build())
            .build(),
        None,
    )?;
    collection.create_index(
        IndexModel::builder().keys(doc! { "visited": 1i32 }).build(),
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
