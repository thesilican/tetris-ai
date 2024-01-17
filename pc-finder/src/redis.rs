use crate::*;
use ::redis::{cmd, Client, Connection};
use anyhow::Result;
use common::*;
use once_cell::sync::Lazy;
use std::{
    collections::{HashMap, HashSet},
    sync::Mutex,
};

static REDIS_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    Mutex::new(client.get_connection().unwrap())
});

pub fn save_tessellations(tessellations: &[Tess]) -> Result<()> {
    println!("Saving tessellations to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let tessellations = tessellations
        .into_iter()
        .map(Pack::pack_base64)
        .collect::<Vec<_>>();
    cmd("DEL").arg("tessellations").query(con)?;
    if tessellations.len() > 0 {
        cmd("RPUSH")
            .arg("tessellations")
            .arg(tessellations)
            .query(con)?;
    }
    Ok(())
}

pub fn load_tessellations() -> Result<Vec<Tess>> {
    println!("Loading tessellations from Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let data = cmd("LRANGE")
        .arg("tessellations")
        .arg("0")
        .arg("-1")
        .query::<Vec<String>>(con)?;
    let mut tessellations = Vec::new();
    for b64 in data {
        let tess = Tess::unpack_base64(&b64)?;
        tessellations.push(tess);
    }
    Ok(tessellations)
}

pub fn record_parent_children(board: PcBoard, children: &[PcBoard]) -> Result<()> {
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let board = board.pack_base64();
    let children = children
        .into_iter()
        .map(Pack::pack_base64)
        .collect::<Vec<_>>();
    // Record children
    if children.len() > 0 {
        cmd("SADD")
            .arg(format!("children:{}", &board))
            .arg(&children)
            .query(con)?;
    }
    // Record parents
    for child in children {
        cmd("SADD")
            .arg(format!("parents:{}", child))
            .arg(&board)
            .query(con)?;
    }
    Ok(())
}

pub fn fetch_children(board: PcBoard) -> Result<Vec<PcBoard>> {
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let board = board.pack_base64();
    let children = cmd("SMEMBERS")
        .arg(format!("children:{}", board))
        .query::<Vec<String>>(con)?;
    let mut res = Vec::new();
    for parent in children {
        res.push(PcBoard::unpack_base64(&parent)?);
    }
    Ok(res)
}

pub fn fetch_parents(board: PcBoard) -> Result<Vec<PcBoard>> {
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let board = board.pack_base64();
    let parents = cmd("SMEMBERS")
        .arg(format!("parents:{}", board))
        .query::<Vec<String>>(con)?;
    let mut res = Vec::new();
    for parent in parents {
        res.push(PcBoard::unpack_base64(&parent)?);
    }
    Ok(res)
}

pub fn save_visited(visited: &HashSet<PcBoard>) -> Result<()> {
    println!("Saving visited to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let visited = visited
        .iter()
        .map(Pack::pack_base64)
        .collect::<Vec<String>>();
    cmd("DEL").arg("visited").query(con)?;
    if visited.len() > 0 {
        cmd("SADD").arg("visited").arg(visited).query(con)?;
    }
    Ok(())
}

pub fn load_visited() -> Result<HashSet<PcBoard>> {
    println!("Loading visited to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let visited = cmd("SMEMBERS")
        .arg("visited")
        .query::<HashSet<String>>(con)?;
    let mut res = HashSet::new();
    for text in visited {
        res.insert(PcBoard::unpack_base64(&text)?);
    }
    Ok(res)
}

pub fn save_pruned(pruned: &HashSet<PcBoard>) -> Result<()> {
    println!("Saving pruned to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pruned = pruned
        .iter()
        .map(Pack::pack_base64)
        .collect::<Vec<String>>();
    cmd("DEL").arg("pruned").query(con)?;
    if pruned.len() > 0 {
        cmd("SADD").arg("pruned").arg(pruned).query(con)?;
    }
    Ok(())
}

pub fn load_pruned() -> Result<HashSet<PcBoard>> {
    println!("Loading pruned to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pruned = cmd("SMEMBERS")
        .arg("pruned")
        .query::<HashSet<String>>(con)?;
    let mut res = HashSet::new();
    for text in pruned {
        res.insert(PcBoard::unpack_base64(&text)?);
    }
    Ok(res)
}

pub fn save_stack(stack: &Vec<PcBoard>) -> Result<()> {
    println!("Saving stack to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let stack = stack.iter().map(Pack::pack_base64).collect::<Vec<_>>();
    cmd("DEL").arg("stack").query(con)?;
    if stack.len() > 0 {
        cmd("RPUSH").arg("stack").arg(stack).query(con)?;
    }
    Ok(())
}

pub fn load_stack() -> Result<Vec<PcBoard>> {
    println!("Loading stack from Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let stack = cmd("LRANGE")
        .arg("stack")
        .arg("0")
        .arg("-1")
        .query::<Vec<String>>(con)?;
    let mut res = Vec::new();
    for text in stack {
        res.push(PcBoard::unpack_base64(&text)?);
    }
    Ok(res)
}

pub fn save_pc_table(pc_table: &HashMap<PcTableKey, PcTableVal>) -> Result<()> {
    println!("Saving pc-table to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pc_table = pc_table
        .iter()
        .map(|(k, v)| [k.pack_base64(), v.pack_base64()])
        .flatten()
        .collect::<Vec<_>>();
    cmd("DEL").arg("pc-table").query(con)?;
    if pc_table.len() > 0 {
        cmd("HSET").arg("pc-table").arg(pc_table).query(con)?;
    }
    Ok(())
}

pub fn load_pc_table() -> Result<HashMap<PcTableKey, PcTableVal>> {
    println!("Loading pc-table from Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pc_table = cmd("HGETALL")
        .arg("pc-table")
        .query::<HashMap<String, String>>(con)?;
    let mut res = HashMap::new();
    for (k, v) in pc_table {
        res.insert(
            PcTableKey::unpack_base64(&k)?,
            PcTableVal::unpack_base64(&v)?,
        );
    }
    Ok(res)
}
