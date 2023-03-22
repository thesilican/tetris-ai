use std::panic::UnwindSafe;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{Builder, JoinHandle};

enum WorkerMsg {
    RunFn(Box<dyn FnOnce(usize) + Send>),
    Quit,
}

enum JobReq<R> {
    Job {
        job: Box<dyn FnOnce() -> R + UnwindSafe + Send>,
        job_id: usize,
    },
    Quit,
}
enum JobRes<R> {
    Success {
        job_id: usize,
        worker_id: usize,
        result: R,
    },
    Panicked {
        worker_id: usize,
    },
}

#[derive(Debug)]
struct Worker {
    pub handle: Option<JoinHandle<()>>,
}
impl Worker {
    pub fn new(id: usize, rx: Receiver<WorkerMsg>) -> Self {
        let func = move || loop {
            let msg = match rx.recv() {
                Ok(msg) => msg,
                // Gracefully exit if main thread panicked
                Err(_) => break,
            };
            match msg {
                WorkerMsg::RunFn(func) => func(id),
                WorkerMsg::Quit => break,
            }
        };
        let handle = Builder::new()
            .name(format!("worker-{id}"))
            .spawn(func)
            .unwrap_or_else(|_| panic!("error spawning worker {id}"));
        Worker {
            handle: Some(handle),
        }
    }
}

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    senders: Vec<Sender<WorkerMsg>>,
}
impl ThreadPool {
    pub fn new(thread_count: usize) -> Self {
        let mut workers = Vec::new();
        let mut senders = Vec::new();
        for i in 0..thread_count {
            let (tx, rx) = channel();
            let worker = Worker::new(i, rx);
            workers.push(worker);
            senders.push(tx);
        }
        ThreadPool { workers, senders }
    }
    pub fn thread_count(&self) -> usize {
        self.workers.len()
    }
    fn setup_job_runners<R: Send + 'static>(
        &self,
    ) -> (Vec<Sender<JobReq<R>>>, Receiver<JobRes<R>>) {
        // Channels for recieving messages from worker
        let (recv_tx, reciever) = channel::<JobRes<R>>();
        let mut senders = Vec::new();
        for i in 0..self.thread_count() {
            let recv_tx = recv_tx.clone();
            // Channels for sending messages to workers
            let (send_tx, send_rx) = channel::<JobReq<R>>();
            senders.push(send_tx);

            // Job runner
            let job_runner = move |worker_id| loop {
                let msg = match send_rx.recv() {
                    Ok(msg) => msg,
                    // Gracefully exit if main thread panicked
                    Err(_) => break,
                };
                match msg {
                    JobReq::Job { job, job_id } => {
                        // Run job
                        let result = std::panic::catch_unwind(job);
                        match result {
                            Ok(result) => {
                                recv_tx
                                    .send(JobRes::Success {
                                        job_id,
                                        worker_id,
                                        result,
                                    })
                                    .ok();
                            }
                            Err(payload) => {
                                recv_tx.send(JobRes::Panicked { worker_id }).ok();
                                std::panic::resume_unwind(payload);
                            }
                        }
                    }
                    JobReq::Quit => break,
                }
            };
            // Send job runner to thread
            self.senders[i]
                .send(WorkerMsg::RunFn(Box::new(job_runner)))
                .unwrap_or_else(|_| panic!("error sending job runner to worker {i}"));
        }
        (senders, reciever)
    }
    pub fn run<R, F>(&self, jobs: Vec<F>) -> Vec<R>
    where
        F: FnOnce() -> R + UnwindSafe + Send + 'static,
        R: Send + 'static,
    {
        let thread_count = self.thread_count();
        if thread_count == 0 {
            // Special case: Complete jobs synchronously
            return jobs.into_iter().map(|f| f()).collect();
        }

        // Set up Job Runners
        let (senders, reciever) = self.setup_job_runners();

        // Start collecting results
        let total = jobs.len();
        let mut completed = 0;
        let mut panicked = None::<usize>;
        let mut jobs = jobs.into_iter().enumerate();
        let mut results = (0..total).map(|_| None::<R>).collect::<Vec<_>>();

        let mut next_job = || match jobs.next() {
            Some((i, job)) => JobReq::Job {
                job: Box::new(job),
                job_id: i,
            },
            None => JobReq::Quit,
        };

        // Send a job to each thread
        for (i, sender) in senders.iter().enumerate() {
            sender
                .send(next_job())
                .unwrap_or_else(|_| panic!("error sending JobMsg to worker {i}"))
        }
        // Recieve job results, and distribute remaining jobs as needed
        while completed < total && panicked.is_none() {
            let res = reciever.recv().expect("error receiving JobReq");
            match res {
                JobRes::Success {
                    job_id,
                    worker_id,
                    result,
                } => {
                    completed += 1;
                    results[job_id] = Some(result);
                    senders[worker_id]
                        .send(next_job())
                        .unwrap_or_else(|_| panic!("error sending JobReq to worker {worker_id}"));
                }
                JobRes::Panicked { worker_id } => {
                    panicked = Some(worker_id);
                    // Tell all other threads to stop
                    for (i, sender) in senders.iter().enumerate() {
                        if i != worker_id {
                            sender
                                .send(JobReq::Quit)
                                .unwrap_or_else(|_| panic!("error sending JobReq to worker {i}"));
                        }
                    }
                }
            }
        }

        if let Some(worker_id) = panicked {
            panic!("worker {worker_id} panicked")
        } else {
            results.into_iter().map(|x| x.unwrap()).collect()
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for sender in self.senders.iter_mut() {
            sender.send(WorkerMsg::Quit).ok();
        }

        for worker in self.workers.iter_mut() {
            if worker.handle.is_some() {
                worker.handle.take().unwrap().join().ok();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_should_run_0_threads() {
        let thread_pool = ThreadPool::new(0);
        let jobs = (0..100).map(|x| move || x * x).collect();
        let result = thread_pool.run(jobs);
        assert_eq!(result, (0..100).map(|x| x * x).collect::<Vec<_>>());
    }

    #[test]
    fn it_should_run_10_threads() {
        let thread_pool = ThreadPool::new(10);
        let jobs = (0..100).map(|x| move || x * x).collect();
        let result = thread_pool.run(jobs);
        assert_eq!(result, (0..100).map(|x| x * x).collect::<Vec<_>>());
    }

    #[test]
    #[should_panic]
    fn it_should_handle_panics() {
        let thread_pool = ThreadPool::new(10);
        let jobs = (0..100)
            .map(|x| move || if x == 50 { panic!() } else { x })
            .collect();
        let _ = thread_pool.run(jobs);
    }
}
