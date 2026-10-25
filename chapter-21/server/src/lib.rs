use std::{sync::{mpsc, Arc, Mutex}, thread};

/// Manages a fixed-size pool of worker threads to execute tasks concurrently.
pub struct ThreadPool {
    workers: Vec<Worker>,
    // The transmitting half of the channel used to send jobs to worker threads.
    sender: mpsc::Sender<Job>,
}

// A type alias for a closure that can be sent across threads, 
// takes no arguments, and has a 'static lifetime.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        // Create a multi-producer, single-consumer channel for jobs.
        let (sender, receiver) = mpsc::channel();
        
        // Wrap the receiver in an Arc and a Mutex so it can be shared 
        // safely across multiple worker threads.
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        // Spawn each worker thread, passing a clone of the receiver Arc to each.
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    
    /// Sends a closure (job) into the channel queue to be picked up 
    /// and executed by an available worker thread.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

/// Represents an individual worker thread in the pool.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    /// Spawns a new worker thread that continuously listens for incoming jobs.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // Lock the mutex to safely acquire exclusive access to the receiver, 
            // then block until the next job is received from the channel.
            let job = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            println!("Worker {} got a job; executing.", id);
            
            // Execute the closure.
            job();
        });

        Worker { id, thread }
    }
}