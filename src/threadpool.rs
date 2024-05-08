use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};


pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job. executing...");
            job();
        });
        Worker {id, thread}
    }
}

impl ThreadPool {
    pub fn new(n_threads: usize) -> ThreadPool {
        assert!(n_threads > 0, "number of threads must be one or higher");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(n_threads);

        for id in 0..n_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
        
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(func);

        self.sender.send(job).unwrap();
        
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
    //TODO stopping threads
    }
}
