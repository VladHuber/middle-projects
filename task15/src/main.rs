use std::error::Error;

use crate::thread_pool::ThreadPool;

mod thread_pool;
mod worker;

fn main() -> Result<(), Box<dyn Error>> {
    let pool = ThreadPool::new(4);
    for i in 0..100 {
        pool.execute(Box::new(move || println!("Task {}", i)));
    }
    Ok(())
}
