use std::sync::{mpsc, Arc, Mutex};
type Worker = Option<std::thread::JoinHandle<()>>;
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            workers.push(spawn(Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

fn spawn(receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    Some(std::thread::spawn(move || loop {
        receiver.lock().unwrap().recv().unwrap()()
    }))
}
