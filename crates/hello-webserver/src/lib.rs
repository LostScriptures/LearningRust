use std::{
    fmt, io,
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SendError},
    },
    thread,
};
// -- ERROR --
type Result<T> = std::result::Result<T, PoolError>;
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum PoolError {
    /// Occures when a 0 balue was passed to the `ThreadPool::new()` function
    WRONG_SIZE_ERROR,
    /// 
    WORKER_CREATION_ERROR(io::Error),
    JOB_SEND_ERROR(SendError<Job>),
}
impl fmt::Display for PoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PoolError::WRONG_SIZE_ERROR => write!(f, "0 is not valid for thread pool size"),
            PoolError::WORKER_CREATION_ERROR(e) => {
                write!(f, "error creating worker thread: {e}")
            }
            PoolError::JOB_SEND_ERROR(e) => {
                write!(f, "error sending Job to thread: {e}")
            }
        }
    }
}

// -- Thread Pool --
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> io::Result<Worker> {
        let builder = thread::Builder::new();
        let thread = builder.spawn(|| {})?;

        Ok(Worker { id, thread })
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Result
    /// Returns a `Result` that is either a new `ThreadPool` or a `PoolError`
    pub fn new(size: usize) -> Result<ThreadPool> {
        if size == 0 {
            return Err(PoolError::WRONG_SIZE_ERROR);
        }

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            match Worker::new(id, Arc::clone(&receiver)) {
                Ok(worker) => workers.push(worker),
                Err(e) => {
                    return Err(PoolError::WORKER_CREATION_ERROR(e));
                }
            }
        }

        Ok(ThreadPool { workers, sender })
    }

    pub fn execute<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        if let Err(e) = self.sender.send(job) {
            return Err(PoolError::JOB_SEND_ERROR(e));
        }
        Ok(())
    }
}
