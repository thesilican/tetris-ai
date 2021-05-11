use std::cmp::min;
use std::marker::PhantomData;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

enum ThreadReq<R> {
    Job {
        job: Box<dyn FnOnce() -> R + Send + 'static>,
        job_id: usize,
    },
    Quit,
}

struct ThreadRes<R> {
    job_id: usize,
    worker_id: usize,
    result: R,
}

struct WorkerThread<R> {
    pub id: usize,
    pub handle: Option<JoinHandle<()>>,
    _r: PhantomData<R>,
}
impl<R> WorkerThread<R>
where
    R: Sized + Send + 'static,
{
    fn new(id: usize, rx: Receiver<ThreadReq<R>>, tx: Sender<ThreadRes<R>>) -> Self {
        // Main thread loop
        let handle = std::thread::spawn(move || loop {
            let (job, job_id) = match rx.recv().unwrap() {
                ThreadReq::Job { job, job_id } => (job, job_id),
                ThreadReq::Quit => break,
            };
            let result = job();
            tx.send(ThreadRes {
                worker_id: id,
                job_id,
                result,
            })
            .unwrap();
        });

        WorkerThread {
            id,
            handle: Some(handle),
            _r: PhantomData,
        }
    }
}

pub struct ThreadPool<R> {
    workers: Vec<WorkerThread<R>>,
    transmitters: Vec<Sender<ThreadReq<R>>>,
    reciever: Receiver<ThreadRes<R>>,
}
impl<R> ThreadPool<R>
where
    R: Sized + Send + 'static,
{
    pub fn new(thread_count: usize) -> Self {
        let mut workers = Vec::new();
        let mut transmitters = Vec::new();
        let (tx, reciever) = mpsc::channel();
        for id in 0..thread_count {
            let (transmitter, rx) = mpsc::channel();
            transmitters.push(transmitter);
            let worker = WorkerThread::<R>::new(id, rx, tx.clone());
            workers.push(worker);
        }
        ThreadPool {
            workers,
            transmitters,
            reciever,
        }
    }
    pub fn run_jobs<F: FnOnce() -> R + Send + 'static>(&mut self, jobs: Vec<F>) -> Vec<R> {
        let num_jobs = jobs.len();
        let num_workers = self.workers.len();
        assert!(num_jobs > 0);

        let mut results: Vec<Option<R>> = jobs.iter().map(|_| None).collect();
        let mut jobs = jobs.into_iter();
        let mut assigned_counter = 0;
        let mut completed_counter = 0;

        for i in 0..min(num_workers, num_jobs) {
            let job = jobs.next().unwrap();
            self.transmitters[i]
                .send(ThreadReq::Job {
                    job: Box::new(job),
                    job_id: assigned_counter,
                })
                .unwrap();
            assigned_counter += 1;
        }
        loop {
            let res = self.reciever.recv().unwrap();
            completed_counter += 1;
            results[res.job_id] = Some(res.result);
            if completed_counter == num_jobs {
                break;
            }
            if assigned_counter < num_jobs {
                let job = jobs.next().unwrap();
                self.transmitters[res.worker_id]
                    .send(ThreadReq::Job {
                        job: Box::new(job),
                        job_id: assigned_counter,
                    })
                    .unwrap();
                assigned_counter += 1;
            }
        }

        results.into_iter().map(|x| x.unwrap()).collect()
    }
    pub fn get_thread_count(&self) -> usize {
        self.workers.len()
    }
}
impl<R> Drop for ThreadPool<R> {
    fn drop(&mut self) {
        for sender in self.transmitters.iter_mut() {
            sender.send(ThreadReq::Quit).unwrap();
        }
        for worker in self.workers.iter_mut() {
            worker.handle.take().unwrap().join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::threading::ThreadPool;
    #[test]

    fn thread_pool_should_work() {
        const NUM_JOBS: usize = 1_000;
        let mut thread_pool = ThreadPool::new(10);
        let mut jobs = Vec::new();
        for i in 0..NUM_JOBS {
            jobs.push(move || i + i);
        }
        let results = thread_pool.run_jobs(jobs);
        for (i, result) in results.into_iter().enumerate() {
            assert_eq!(i + i, result);
        }
    }
}
