use crate::PcBoard;
use anyhow::Result;
use libtetris::Pack;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::File,
    io::{Read, Write},
};
use tinyvec::TinyVec;

fn prune_bfs(edges: Vec<(PcBoard, PcBoard)>) -> Vec<(PcBoard, PcBoard)> {
    let mut backlinks = HashMap::<PcBoard, TinyVec<[PcBoard; 4]>>::new();
    for &(parent, child) in edges.iter() {
        backlinks.entry(child).or_default().push(parent);
    }

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(PcBoard::new());

    while let Some(child) = queue.pop_front() {
        if visited.contains(&child) {
            continue;
        }
        visited.insert(child);
        println!(
            "{}\n  Queue: {:>8}\nVisited: {:>8}\n",
            child,
            queue.len(),
            visited.len()
        );
        let Some(parents) = backlinks.get(&child) else {
            continue;
        };
        for &parent in parents {
            if !visited.contains(&parent) {
                queue.push_back(parent);
            }
        }
    }

    let mut pruned_edges = Vec::new();
    for (parent, child) in edges {
        if visited.contains(&parent) && visited.contains(&child) {
            pruned_edges.push((parent, child));
        }
    }

    pruned_edges
}

pub fn prune_graph(edges: Vec<(PcBoard, PcBoard)>) -> Result<Vec<(PcBoard, PcBoard)>> {
    println!("Pruning graph edges");
    let file = File::open("data/pruned.bin");
    if let Ok(mut file) = file {
        println!("Reading graph edges from data/pruned.bin");
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        let output = Vec::<(PcBoard, PcBoard)>::unpack_bytes(&data)?;
        return Ok(output);
    }

    let output = prune_bfs(edges);

    println!("Saving pruned edges to data/pruned.bin");
    let bytes = output.pack_bytes();
    let mut file = File::create("data/pruned.bin")?;
    file.write_all(&bytes)?;

    Ok(output)
}
