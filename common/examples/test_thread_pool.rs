use anyhow::Result;
use common::*;

enum Req {
    Task(usize),
    Quit,
}
struct Res {
    task: usize,
    result: usize,
}

fn handler(ctx: &Context<Req, Res>) -> Result<()> {
    while let Req::Task(num) = ctx.recv()? {
        let result = if num % 2 == 0 { num / 2 } else { 3 * num + 1 };
        println!("Thread {} task {num}", ctx.thread_id());
        ctx.send(Res { task: num, result })?;
    }
    Ok(())
}

fn main() -> Result<()> {
    let thread_count = 10;
    let mut thread_pool = ThreadPool::start(thread_count, handler);
    let mut current = 0;
    let job_count = 1_000_000;
    let mut results = vec![None::<usize>; job_count];
    for i in 0..thread_count {
        thread_pool.send(i, Req::Task(i))?;
        current += 1;
    }
    while let Some((thread_id, msg)) = thread_pool.recv()? {
        let Res { task, result } = msg;
        results[task] = Some(result);
        if current == job_count {
            thread_pool.send(thread_id, Req::Quit)?;
        } else {
            thread_pool.send(thread_id, Req::Task(current))?;
            current += 1;
        }
    }
    Ok(())
}
