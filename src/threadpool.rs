use std::{
    error::Error,
    sync::{
        mpsc::{self, Receiver},
        Arc,
        Mutex
    },
    thread
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    tx: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(worker_count: usize) -> Self {
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(worker_count);

        for _ in 0..worker_count {
            let worker = Worker::new(rx.clone());
            workers.push(worker);
        }

        Self {
            workers,
            tx
        }
    }
    pub fn execute<T>(&self, job: T) -> Result<(), Box<dyn Error>> 
    where
        T: FnOnce() + Send + 'static
    {
        let job = Box::new(job);
        self.tx.send(job)?;
        Ok(())
    }
}

struct Worker {
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(rx: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            loop {
                let job = rx.lock().unwrap().recv().unwrap();
                job()
            }
        });

        Worker {
            thread
        }
    }
}
