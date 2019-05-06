use std::fmt;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// threads is the lib for the ThreadPool struct

#[doc(inline)]
pub use super::error::Result;

use super::error::{Error, ErrorKind};

/// struct used a type for the ThreadPool
pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    do_verbose: bool,
}

impl ThreadPool {
    /// creates a new `ThreadPool` with the given number of threads
    ///
    /// # Arguments
    /// * `size` - number of threads to create.
    ///
    /// # Errors
    /// Errors if the number of threads to create is equal to 0.
    ///
    /// # Examples
    ///
    /// ## Create a ThreadPool with 4 threads
    /// ```
    /// use poke_escape_server::threads::ThreadPool;
    /// let pool = ThreadPool::new(4).unwrap();  // creates a threadpool with 4 threads
    /// ```
    ///
    /// ## Fails when trying to create 4 threads
    /// ```should_panic
    /// use poke_escape_server::threads::ThreadPool;
    /// let pool = ThreadPool::new(0).unwrap();     // unwrap panics
    /// ```
    pub fn new(size: usize) -> Result<ThreadPool> {
        if size == 0 {
            return Err(Error::new(ErrorKind::PoolToSmall));
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            size,
            sender,
            do_verbose: false,
        })
    }

    /// set ThreadPool into verbose mode
    ///
    /// will print output like wich thread is dropped if set to true
    ///
    /// # Example
    /// ```
    /// use poke_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// pool.verbose();
    /// assert_eq!(pool.is_verbose(), true);
    /// ```
    pub fn verbose(&mut self) -> &Self {
        self.do_verbose = true;
        println!("running ThreadPool in verbose mode"); //TODO: colors
        self
    }

    /// sets ThreadPool in the given verbose mode
    ///
    /// # Example
    /// ## set into verbose mode
    /// ```
    /// use poke_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// pool.set_verbose_mode(true);
    /// assert_eq!(pool.is_verbose(), true);
    /// ```
    ///
    /// ## set out of verbose mode
    /// ```
    /// use poke_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// pool.set_verbose_mode(false);
    /// assert_eq!(pool.is_verbose(), false);
    /// ```
    pub fn set_verbose_mode(&mut self, mode: bool) -> &Self {
        self.do_verbose = mode;
        self
    }

    /// checks if the ThreadPool is running in verbose mode
    ///
    /// # Example
    /// ```
    /// use poke_escape_server::threads::ThreadPool;
    /// let mut pool = ThreadPool::new(4).unwrap();
    /// pool.verbose();
    /// assert_eq!(pool.is_verbose(), true);
    /// ```
    pub fn is_verbose(&self) -> bool {
        self.do_verbose
    }

    /// returns the number of Threads in the ThreadPool
    ///
    /// # Examples
    ///
    /// ```
    /// use poke_escape_server::threads::ThreadPool;
    /// let pool = ThreadPool::new(4).unwrap();
    /// assert_eq!(pool.get_threads(), 4);
    /// ```
    pub fn get_threads(&self) -> usize {
        self.size
    }

    /// execute send a function into a thread to be executed there
    pub fn execute<'a, F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        /*match self.sender.send(Message::NewJob(job)) {
            //FIXME: unwrap
            Ok(()) => return Ok(()),
            Err(err) => return Err(err.to_string()),
        };*/
        Ok(self.sender.send(Message::NewJob(job))?)
    }
}

impl fmt::Display for ThreadPool {
    /// standart formater for print! macro
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadPool with {} workers", self.size) // TODO: Colors
    }
}

impl Drop for ThreadPool {
    /// signals each thread to stop befor droping itself
    fn drop(&mut self) {
        if self.do_verbose {
            println!("signaling workers to stop");
        }
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap(); //FIXME: unwrap
        }

        for worker in &mut self.workers {
            if self.do_verbose {
                println!("Stopping worker {}", worker.id);
            }
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap(); //FIXME: unwrap
            }
        }
    }
}

/// implement function to convert sendError to Error
impl std::convert::From<std::sync::mpsc::SendError<super::threads::Message>> for Error {
    fn from(err: std::sync::mpsc::SendError<super::threads::Message>) -> Self {
        match err.0 {
            Message::NewJob(_) => Error::new(ErrorKind::PoolSendError(true)),
            Message::Terminate => Error::new(ErrorKind::PoolSendError(false)),
        }
    }
}

/// enum to hold either a Job or a Terminate signal for the worker thread
enum Message {
    NewJob(Job),
    Terminate,
}

// Worker represents a Thread in the ThreadPool
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap(); //FIXME: unwrap

            match message {
                Message::NewJob(job) => {
                    job.call_box();
                }
                Message::Terminate => {
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// trait to be used as type for the Job
trait FnBox {
    fn call_box(self: Box<Self>);
}

/// Type for the Job to send to a worker
type Job = Box<FnBox + Send + 'static>;

/// implementation of FnBox for the job type
impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
