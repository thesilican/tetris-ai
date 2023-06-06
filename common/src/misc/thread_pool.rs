use anyhow::{anyhow, bail, Result};
use crossbeam::channel::{unbounded, Receiver, Sender};
use std::thread::{spawn, JoinHandle};

enum ThreadRes<N> {
    Msg { thread_id: usize, msg: N },
    Join { thread_id: usize },
}

pub struct Context<M, N> {
    thread_id: usize,
    sender: Sender<ThreadRes<N>>,
    receiver: Receiver<M>,
}
impl<M, N> Context<M, N> {
    pub fn thread_id(&self) -> usize {
        self.thread_id
    }
    pub fn send(&self, msg: N) -> Result<()> {
        let msg = ThreadRes::Msg {
            thread_id: self.thread_id,
            msg,
        };
        self.sender
            .send(msg)
            .map_err(|_| anyhow!("error sending message"))
    }
    pub fn recv(&self) -> Result<M> {
        self.receiver
            .recv()
            .map_err(|_| anyhow!("error receiving message"))
    }
}

enum Worker {
    Running(JoinHandle<Result<()>>),
    Finished,
}
impl Worker {
    fn new<F, M, N>(
        f: F,
        thread_id: usize,
        sender: Sender<ThreadRes<N>>,
        receiver: Receiver<M>,
    ) -> Self
    where
        F: Fn(&Context<M, N>) -> Result<()> + Send + 'static,
        M: Send + 'static,
        N: Send + 'static,
    {
        let ctx = Context {
            thread_id,
            sender,
            receiver,
        };
        let handle = spawn(move || {
            let res = f(&ctx);
            ctx.sender
                .send(ThreadRes::Join {
                    thread_id: ctx.thread_id,
                })
                .map_err(|_| anyhow!("error sending message"))?;
            res
        });
        Worker::Running(handle)
    }
    fn is_finished(&self) -> bool {
        match self {
            Worker::Running(_) => false,
            Worker::Finished => true,
        }
    }
    fn join(&mut self) -> Result<()> {
        let val = std::mem::replace(self, Worker::Finished);
        match val {
            Worker::Running(handle) => {
                let res = handle
                    .join()
                    .map_err(|_| anyhow!("thread failed to join"))?;
                res
            }
            Worker::Finished => panic!("attempted to join completed worker"),
        }
    }
}

pub struct ThreadPool<M, N> {
    workers: Vec<Worker>,
    senders: Vec<Sender<M>>,
    receiver: Receiver<ThreadRes<N>>,
}

impl<M, N> ThreadPool<M, N> {
    pub fn start<F>(thread_count: usize, f: F) -> Self
    where
        F: Fn(&Context<M, N>) -> Result<()> + Send + Copy + 'static,
        M: Send + 'static,
        N: Send + 'static,
    {
        let mut workers = Vec::new();
        let mut senders = Vec::new();
        let (sender, receiver) = unbounded();
        for i in 0..thread_count {
            let (worker_sender, worker_receiver) = unbounded();
            let worker = Worker::new(f, i, sender.clone(), worker_receiver);
            workers.push(worker);
            senders.push(worker_sender);
        }
        ThreadPool {
            workers,
            senders,
            receiver,
        }
    }
    pub fn send(&self, thread_id: usize, msg: M) -> Result<()> {
        if thread_id >= self.senders.len() {
            bail!("thread_id {thread_id} is out of bounds");
        }
        if self.workers[thread_id].is_finished() {
            bail!("worker {thread_id} has finished");
        }
        self.senders[thread_id]
            .send(msg)
            .map_err(|_| anyhow!("error sending message"))
    }
    pub fn recv(&mut self) -> Result<Option<(usize, N)>> {
        loop {
            let msg = self
                .receiver
                .recv()
                .map_err(|_| anyhow!("error receiving message"))?;
            match msg {
                ThreadRes::Msg { thread_id, msg } => return Ok(Some((thread_id, msg))),
                ThreadRes::Join { thread_id } => {
                    self.workers[thread_id].join()?;
                    if self.active_workers() == 0 {
                        return Ok(None);
                    }
                }
            }
        }
    }
    pub fn join(mut self) -> Result<()> {
        let mut fail_count = 0;
        for worker in self.workers.iter_mut() {
            if !worker.is_finished() {
                match worker.join() {
                    Ok(_) => {}
                    Err(_) => fail_count += 1,
                }
            }
        }
        if fail_count > 0 {
            bail!("{} threads failed to join or errored", fail_count);
        } else {
            Ok(())
        }
    }
    pub fn active_workers(&self) -> usize {
        self.workers.iter().filter(|w| !w.is_finished()).count()
    }
}

impl<M, N> Drop for ThreadPool<M, N> {
    fn drop(&mut self) {
        if self.active_workers() > 0 {
            panic!("thread pool dropped with active workers, please call thread_pool.join() to ensure that all threads have joined");
        }
    }
}
