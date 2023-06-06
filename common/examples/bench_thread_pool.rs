use std::time::Instant;

use anyhow::{bail, Result};
use common::*;

enum Req {
    Ping,
    Quit,
}

enum Res {
    Pong,
}

fn handler(ctx: &Context<Req, Res>) -> Result<()> {
    loop {
        match ctx.recv()? {
            Req::Ping => ctx.send(Res::Pong)?,
            Req::Quit => break,
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let start = Instant::now();
    let thread_count = 100;
    let mut thread_pool = ThreadPool::start(thread_count, handler);
    let mut count = 0;
    for i in 0..thread_count {
        thread_pool.send(i, Req::Ping)?;
    }
    loop {
        match thread_pool.recv() {
            Ok(Some((i, _))) => {
                if count == 1_000_000 {
                    thread_pool.send(i, Req::Quit)?;
                } else {
                    thread_pool.send(i, Req::Ping)?;
                    count += 1;
                }
            }
            Ok(None) => break,
            _ => bail!("something went horribly wrong"),
        }
    }
    let end = start.elapsed();
    println!("{:?}", end);
    Ok(())
}
