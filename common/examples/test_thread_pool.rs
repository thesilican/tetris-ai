use common::misc::ThreadPool;

fn main() {
    let mut thread_pool = ThreadPool::new(50);
    let jobs = (0..1000).map(|x| move || x + x).collect();
    let _ = thread_pool.run(jobs);
    println!("Finished Jobs 1");
    let jobs = (0..1000).map(|_| || true).collect();
    let _ = thread_pool.run(jobs);
    println!("Finished Jobs 2");
}
