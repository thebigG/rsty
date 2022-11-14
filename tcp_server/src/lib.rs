use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Job;

impl Worker{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let thread = thread::spawn(|| {
            receiver;
        });
        Worker{id, thread}
    }
}
pub struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool{

   /// Create a new ThreadPool
   ///
   /// The size is the number of threads in the pool.
   /// # Panics
   /// The 'new' function will panic if size is zero.
   pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

       let (sender, receiver) = mpsc::channel();
       let mut workers = Vec::with_capacity(size);

       let receiver = Arc::new(Mutex::new(receiver));
       for id in 0..size{
           workers.push(Worker::new(id, Arc::clone(&receiver)));
       }
        ThreadPool{workers, sender}
    }

    pub fn execute<F>(&self, f: F)
        where
        F: FnOnce() + Send + 'static
        {

        }
    }
