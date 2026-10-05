use std::sync::{Arc, Mutex};
use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};
use crate::thread_pool::Job;
use std::time::Duration;
pub struct Worker {
    pub thread: Option<JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let join_handle = thread::spawn(move || {
            loop {
                let job = {
                    let lock = receiver.lock().unwrap();
                    lock.recv()
                };
                match job {
                    Ok(job) => {
                        println!("Worker {}", id);
                        job();
                        thread::sleep(Duration::from_millis(300));
                    }
                    Err(_) => break,
                }
            }
        });
        Worker {
            thread: Some(join_handle),
        }
    }
}
