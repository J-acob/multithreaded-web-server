use std::{sync::{mpsc, Arc, Mutex}, thread};

/// A pool of threads which manage running various threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(thread_count: usize) -> ThreadPool {
        assert!(thread_count > 0);

        let (sender, receiver) = mpsc::channel();

        // Create wrapper around receiver so we can send it between channels safely
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(thread_count);

        for id in 0..thread_count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }   

        ThreadPool {workers, sender: Some(sender)}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        // Drop the sender, which closes the channel so nothing can be received anymore
        drop(self.sender.take());

        // Join the threads
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {

        let thread = thread::spawn(move || loop {
            
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {

                    println!("Worker {id} got a job; executing.");

                    job();
                },
                Err(_) => {

                    // receiving an error 
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            };
        });

        Worker { id, thread: Some(thread) }
    }
}