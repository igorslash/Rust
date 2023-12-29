use std::sync::{Arc, mpsc, Mutex};
use std::thread;
/// The `ThreadPool` struct represents a pool of worker threads that
/// can execute jobs concurrently.
/// It uses a channel to communicate between the main thread and the
/// worker threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}
/// The `Job` type represents a closure that
/// can be executed by the worker threads.
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new `ThreadPool` with the specified number of worker threads.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of worker threads to create in the pool.
    ///
    /// # Panics
    ///
    /// The `size` argument must be greater than 0. If it is 0 or less,
    /// `assert!` will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = ThreadPool::new(4);
    /// ```
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        /// Create a channel for communication between the main thread
        /// and worker threads.
        let (sender, receiver) = mpsc::channel();
        /// Wrap the receiver in an `Arc` and `Mutex` to share it between
        /// worker threads.
        let receiver = Arc::new(Mutex::new(receiver));
        /// Create the specified number of worker threads and store
        /// them in a vector.
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    /// Executes a closure in one of the worker threads.
    ///
    /// # Arguments
    ///
    /// * `f` - The closure to be executed by the worker thread.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = ThreadPool::new(4);
    /// pool.execute(|| {
    ///     println!("Hello from a worker thread!");
    /// });
    /// ```

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }}
impl Drop for ThreadPool {
    /// Shuts down the `ThreadPool` by joining all worker threads.
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
/// The `Worker` struct represents a worker thread in the `ThreadPool`.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    /// Creates a new `Worker` with the specified ID and receiver for jobs.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the worker thread.
    /// * `receiver` - The receiver for jobs that the worker thread will execute.
    ///
    /// # Examples
    ///
    /// ```
    /// let receiver = Arc::new(Mutex::new(receiver));
    /// let worker = Worker::new(0, Arc::clone(&receiver));
    /// ```
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job();
        });
        Worker { id, thread: Some(thread) }
    }
}