use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{Builder, JoinHandle};

enum WorkerMsg {
    RunFn(Box<dyn FnOnce(usize) -> () + Send>),
    Quit,
}

enum JobReq<R> {
    Job {
        job: Box<dyn FnOnce() -> R + Send>,
        job_id: usize,
    },
    Quit,
}
struct JobRes<R> {
    job_id: usize,
    worker_id: usize,
    result: R,
}

#[derive(Debug)]
struct Worker {
    pub id: usize,
    pub handle: Option<JoinHandle<()>>,
}
impl Worker {
    pub fn new(id: usize, rx: Receiver<WorkerMsg>) -> Self {
        let func = move || loop {
            let msg = rx.recv().expect("Error recieving WorkerMsg");
            match msg {
                WorkerMsg::RunFn(func) => func(id),
                WorkerMsg::Quit => break,
            }
        };
        let handle = Builder::new()
            .name(format!("worker-{}", id))
            .spawn(func)
            .expect(&format!("Error spawning worker {}", id));
        Worker {
            id,
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
    pub fn run<R, F>(&mut self, jobs: Vec<F>) -> Vec<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let thread_count = self.thread_count();
        let job_count = jobs.len();
        if thread_count == 0 {
            // Special case: Complete jobs synchronously
            return jobs.into_iter().map(|f| f()).collect();
        }
        // Set up Job Runners
        // (Function to handle jobs)
        let (recv_tx, reciever) = channel::<JobRes<R>>();
        let mut senders = Vec::new();
        for i in 0..thread_count {
            let recv_tx = recv_tx.clone();
            let (send_tx, send_rx) = channel::<JobReq<R>>();
            senders.push(send_tx);
            // Job runner
            let job_runner = move |worker_id| loop {
                let msg = send_rx.recv().expect("Error recieving JobMsg");
                match msg {
                    JobReq::Job { job, job_id } => {
                        // Run job
                        let result = job();
                        recv_tx
                            .send(JobRes {
                                job_id,
                                worker_id,
                                result,
                            })
                            .expect("Error sending JobRes");
                    }
                    JobReq::Quit => break,
                }
            };
            // Send job runner to thread
            self.senders[i]
                .send(WorkerMsg::RunFn(Box::new(job_runner)))
                .expect(&format!("Error sending job runner to worker {}", i));
        }

        let mut results = (0..job_count).map(|_| None).collect::<Vec<Option<R>>>();
        let mut results_count = 0;
        let mut jobs = jobs.into_iter().enumerate();
        // Helper function to get the next JobMsg
        let mut next_job_msg = || match jobs.next() {
            Some((i, job)) => JobReq::Job {
                job: Box::new(job),
                job_id: i,
            },
            None => JobReq::Quit,
        };
        // Send a job to each thread
        for i in 0..thread_count {
            let msg = next_job_msg();
            senders[i]
                .send(msg)
                .expect(&format!("Error sending JobMsg to worker {}", i))
        }
        // Recieve job results, and distribute remaining jobs as needed
        while results_count < job_count {
            let JobRes {
                job_id,
                worker_id,
                result,
            } = reciever.recv().expect("Error recieving JobReq");
            let msg = next_job_msg();
            senders[worker_id]
                .send(msg)
                .expect(&format!("Error sending JobMsg to worker {}", worker_id));
            results[job_id] = Some(result);
            results_count += 1
        }
        // All results should have been recieved at this point
        results.into_iter().map(|x| x.unwrap()).collect()
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        for sender in self.senders.iter_mut() {
            sender.send(WorkerMsg::Quit).unwrap();
        }
        // Join threads, keeping track of numbers if there is an error
        let mut errors = Vec::new();
        for (i, worker) in self.workers.iter_mut().enumerate() {
            let res = worker.handle.take().unwrap().join();
            if res.is_err() {
                errors.push(i);
            }
        }
        if !errors.is_empty() {
            let threads = errors
                .into_iter()
                .map(|x| format!("{}", x))
                .reduce(|mut a, v| {
                    a.push_str(&v);
                    a
                })
                .unwrap();
            panic!("Could not join threads: {} (panicked?)", threads);
        }
    }
}
