use common::*;

fn main() {
    let thread_pool = ThreadPool::new(20);
    let jobs = (0..1000)
        .map(|x| {
            move || {
                std::thread::sleep(std::time::Duration::from_millis(x * 1234 % 1000));
                println!("{}", x);
                if x == 100 {
                    panic!()
                }
            }
        })
        .collect::<Vec<_>>();
    thread_pool.run(jobs);
    // let _ = thread_pool.run(jobs);
    // println!("Finished Jobs 1");
    // let jobs = (0..1000).map(|_| || true).collect();
    // let _ = thread_pool.run(jobs);
    // println!("Finished Jobs 2");
    // let jobs = (0..1000)
    //     .map(|x| move || if x == 100 { panic!() } else { x })
    //     .collect();
    // let _ = thread_pool.run(jobs);
}
