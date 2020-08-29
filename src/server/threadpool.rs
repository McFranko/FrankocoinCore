#![allow(non_snake_case)]
#![allow(dead_code)]

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: std::sync::mpsc::Sender<Job>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = std::sync::mpsc::channel();
        let receiver = std::sync::Arc::new(std::sync::Mutex::new(receiver));

        for id in 0..size
        {
            workers.push(Worker::new(id, std::sync::Arc::clone(&receiver)));
        }

        return ThreadPool { workers, sender };
    }
    pub fn execute<Function>(&self, function: Function)
    where
        Function: FnOnce() + Send + 'static,
    {
        let job =  Box::new(function);
        self.sender.send(job).unwrap();

    }
}

struct Worker
{
    id: usize,
    thread: std::thread::JoinHandle<()>
}

impl Worker
{
    fn new(id: usize, receiver: std::sync::Arc<std::sync::Mutex<std::sync::mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = std::thread::spawn(move || loop
        {
            let job = receiver.lock().unwrap().recv().unwrap();
            job();
        });

        return Worker {id, thread};
    }
}
