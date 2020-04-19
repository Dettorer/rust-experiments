use std::io::prelude::*;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect(&format!(
                    "Worker {} error while locking the message receiver",
                    id
                ))
                .recv()
                .expect(&format!("Worker {} error while receiving a message", id));

            match message {
                Message::NewJob(job) => job(),
                Message::Terminate => break,
            }
        });

        Worker {
            id: id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// # Arguments
    ///
    /// * `size` - The maximum number of threads in the pool.
    ///
    /// # Panics
    ///
    /// This will panic if `size` is `0`.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::NewJob(Box::new(f));
        self.sender
            .send(job)
            .expect("Could not send job to the pool");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Could not send 'Terminate' to workers");
        }

        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                print!("Shutting down worker {}... ", worker.id);
                std::io::stdout().flush();
                print!(
                    "{}",
                    match thread.join() {
                        Ok(_) => format!("ok"),
                        Err(e) => format!("giving up: ({:?})", e),
                    }
                );
                println!();
            }
        }
    }
}
