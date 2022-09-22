use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool
{
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        let mut workers = Vec::with_capacity(size);
        
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size
        {
            workers.push(Worker::new(i, Arc::clone(&receiver)))
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers
        {
            if let Some(thread) = worker.thread.take()
            {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker
{
    _id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker
    {
        let thread = thread::spawn(move || loop {
            match receiver
            .lock()
            .unwrap()
            .recv() {
                Ok(job) => job(),
                Err(_) => break
            }
        });
        Worker { _id: id, thread: Some(thread) }
    }
}