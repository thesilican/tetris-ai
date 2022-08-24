use crate::*;
use ::redis::{cmd, Client, Connection};
use common::*;
use std::{
    collections::{HashMap, HashSet},
    lazy::Lazy,
    sync::Mutex,
};

static REDIS_CONNECTION: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let client = Client::open("redis://127.0.0.1/").unwrap();
    Mutex::new(client.get_connection().unwrap())
});

pub fn save_tessellations(tessellations: &[Tess]) -> GenericResult<()> {
    println!("Saving tessellations to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let tessellations = tessellations
        .into_iter()
        .map(SerdeBytes::base64_serialize)
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

pub fn load_tessellations() -> GenericResult<Vec<Tess>> {
    println!("Loading tessellations from Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let data = cmd("LRANGE")
        .arg("tessellations")
        .arg("0")
        .arg("-1")
        .query::<Vec<String>>(con)?;
    let mut tessellations = Vec::new();
    for b64 in data {
        let tess = Tess::base64_deserialize(&b64)?;
        tessellations.push(tess);
    }
    Ok(tessellations)
}

pub fn record_parent_children(board: PcBoard, children: &[PcBoard]) -> GenericResult<()> {
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let board = board.base64_serialize();
    let children = children
        .into_iter()
        .map(SerdeBytes::base64_serialize)
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

pub fn fetch_children(board: PcBoard) -> GenericResult<Vec<PcBoard>> {
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let board = board.base64_serialize();
    let children = cmd("SMEMBERS")
        .arg(format!("children:{}", board))
        .query::<Vec<String>>(con)?;
    let mut res = Vec::new();
    for parent in children {
        res.push(PcBoard::base64_deserialize(&parent)?);
    }
    Ok(res)
}

pub fn fetch_parents(board: PcBoard) -> GenericResult<Vec<PcBoard>> {
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let board = board.base64_serialize();
    let parents = cmd("SMEMBERS")
        .arg(format!("parents:{}", board))
        .query::<Vec<String>>(con)?;
    let mut res = Vec::new();
    for parent in parents {
        res.push(PcBoard::base64_deserialize(&parent)?);
    }
    Ok(res)
}

pub fn save_visited(visited: &HashSet<PcBoard>) -> GenericResult<()> {
    println!("Saving visited to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let visited = visited
        .iter()
        .map(SerdeBytes::base64_serialize)
        .collect::<Vec<String>>();
    cmd("DEL").arg("visited").query(con)?;
    if visited.len() > 0 {
        cmd("SADD").arg("visited").arg(visited).query(con)?;
    }
    Ok(())
}

pub fn load_visited() -> GenericResult<HashSet<PcBoard>> {
    println!("Loading visited to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let visited = cmd("SMEMBERS")
        .arg("visited")
        .query::<HashSet<String>>(con)?;
    let mut res = HashSet::new();
    for text in visited {
        res.insert(PcBoard::base64_deserialize(&text)?);
    }
    Ok(res)
}

pub fn save_pruned(pruned: &HashSet<PcBoard>) -> GenericResult<()> {
    println!("Saving pruned to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pruned = pruned
        .iter()
        .map(SerdeBytes::base64_serialize)
        .collect::<Vec<String>>();
    cmd("DEL").arg("pruned").query(con)?;
    if pruned.len() > 0 {
        cmd("SADD").arg("pruned").arg(pruned).query(con)?;
    }
    Ok(())
}

pub fn load_pruned() -> GenericResult<HashSet<PcBoard>> {
    println!("Loading pruned to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pruned = cmd("SMEMBERS")
        .arg("pruned")
        .query::<HashSet<String>>(con)?;
    let mut res = HashSet::new();
    for text in pruned {
        res.insert(PcBoard::base64_deserialize(&text)?);
    }
    Ok(res)
}

pub fn save_stack(stack: &Vec<PcBoard>) -> GenericResult<()> {
    println!("Saving stack to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let stack = stack
        .iter()
        .map(SerdeBytes::base64_serialize)
        .collect::<Vec<_>>();
    cmd("DEL").arg("stack").query(con)?;
    if stack.len() > 0 {
        cmd("RPUSH").arg("stack").arg(stack).query(con)?;
    }
    Ok(())
}

pub fn load_stack() -> GenericResult<Vec<PcBoard>> {
    println!("Loading stack from Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let stack = cmd("LRANGE")
        .arg("stack")
        .arg("0")
        .arg("-1")
        .query::<Vec<String>>(con)?;
    let mut res = Vec::new();
    for text in stack {
        res.push(PcBoard::base64_deserialize(&text)?);
    }
    Ok(res)
}

pub fn save_pc_table(pc_table: &HashMap<PcTableKey, PcTableVal>) -> GenericResult<()> {
    println!("Saving pc-table to Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pc_table = pc_table
        .iter()
        .map(|(k, v)| [k.base64_serialize(), v.base64_serialize()])
        .flatten()
        .collect::<Vec<_>>();
    cmd("DEL").arg("pc-table").query(con)?;
    if pc_table.len() > 0 {
        cmd("HSET").arg("pc-table").arg(pc_table).query(con)?;
    }
    Ok(())
}

pub fn load_pc_table() -> GenericResult<HashMap<PcTableKey, PcTableVal>> {
    println!("Loading pc-table from Redis");
    let con = &mut *REDIS_CONNECTION.lock().unwrap();
    let pc_table = cmd("HGETALL")
        .arg("pc-table")
        .query::<HashMap<String, String>>(con)?;
    let mut res = HashMap::new();
    for (k, v) in pc_table {
        res.insert(
            PcTableKey::base64_deserialize(&k)?,
            PcTableVal::base64_deserialize(&v)?,
        );
    }
    Ok(res)
}
