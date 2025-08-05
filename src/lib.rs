use std::sync::{mpsc, Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
trait FnBox {
    fn call_box(self: Box<Self>);
}

type Job = Box<dyn FnBox + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<Self>) {
        (*self)();
    }
}


pub struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>
}


impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Message>>>) -> Self {
        let thread =  thread::spawn(move || {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();
                
                
                match message { 
                    Message::NewJob(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job.call_box()
                    },
                    Message::Terminate => {
                        println!("Worker {} was told to terminate.", id);
                        break;
                    }
                }
            }
        });
        
        Self {
            id,
            thread: Some(thread)
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>
}

impl ThreadPool {
    /// ### Create a new thread pool
    /// 
    /// Size: the size is the number of threads in the pool
    /// 
    /// This function panics if the size is zero
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        
        let (sender, receiver) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(receiver));
        
        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        
        for id in 0..size {
            // create some threads and store them in the vector.
            workers.push(Worker::new(id, Arc::clone(&receiver)))
        }
        
        Self {
            workers,
            sender
        }
    }
    
    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate messages");
        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }
        
        println!("Shutting down workers");
        for worker in &mut self.workers {
            
            if let Some(thread) = worker.thread.take() { 
                thread.join().unwrap();
            }
        }
    }
}