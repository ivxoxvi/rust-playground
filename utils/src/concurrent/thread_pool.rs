use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id.to_string(), Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn exec<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            if let Err(e) = worker.thread.join() {
                eprintln!("Worker {} panicked during execution: {:?}", worker.id, e);
            }
        }
    }
}
struct Worker {
    id: String,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(id: String, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let id_clone = id.clone();
        let thread = thread::spawn(move || {
            loop {
                // lock will release when this statment end, this is important.
                let msg = receiver.lock().unwrap().recv();
                match msg {
                    Ok(job) => {
                        println!("<{id_clone}>-[start]");
                        job();
                        println!("<{id_clone}>---[end]");
                    }
                    Err(_) => {
                        println!("Worker {id_clone} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::*;

    #[test]
    fn ttt() {
        let pool = ThreadPool::new(5);
        pool.exec(|| {
            println!("go");
            sleep(Duration::from_secs(4));
            println!("okl")
        });
        pool.exec(|| println!("cnmd"));
        pool.exec(|| println!("cnmd"));
        thread::sleep(Duration::from_secs(5));
    }
}
