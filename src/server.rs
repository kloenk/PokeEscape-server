use std::fmt;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    size: usize,
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    do_verbose: bool,
}

impl ThreadPool {
    pub fn new(size: usize) -> Result<ThreadPool, String> {
        if size == 0 {
            return Err(String::from("Size cannot be 0"));
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

    pub fn verbose(mut self) -> Self {
        self.do_verbose = true;
        self
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl fmt::Display for ThreadPool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadPool with {} workers", self.size)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if self.do_verbose {
            println!("signaling workers to stop");
        }
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if self.do_verbose {
                println!("Stopping worker {}", worker.id);
            }
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

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

type Job = Box<FnBox + Send + 'static>;

trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}