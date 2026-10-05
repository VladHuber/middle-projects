use crate::worker::Worker;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Mutex};

use std::ops::Drop;

pub type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}
impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        let (tx, rx) = mpsc::channel();
        let mut workers = Vec::new();
        let receivers = Arc::new(Mutex::new(rx));
        for id in 1..=size {
            let receiver = Arc::clone(&receivers);
            let worker = Worker::new(id, receiver);
            workers.push(worker);
        }
        ThreadPool {
            workers,
            sender: Some(tx),
        }
    }
    pub fn execute(&self, msg: Job) {
        if let Some(sender) = self.sender.as_ref() {
            sender.send(msg).unwrap();
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();
        for work in &mut self.workers{
            if let Some(thread) = work.thread.take(){
                thread.join().unwrap();
            }
        }
    }
}
