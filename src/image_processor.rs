use std::error::Error;

use crate::{batch::BatchCoordinator, threadpool::ThreadPool};

pub struct ImageProcessor {
    pub jobs_processed: u32,
    pub thread_pool: ThreadPool,
}

impl ImageProcessor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ImageProcessor {
            jobs_processed: 0,
            thread_pool: ThreadPool::new(num_cpus::get()).unwrap()
        })
    }

    pub fn processor_loop(&mut self, coordinator: BatchCoordinator) {
        for batch_job in coordinator.job_queue.lock().unwrap().iter() {
            println!("{:?}", batch_job.arguments);

            self.jobs_processed += 1;
        }
        println!("Images processed: {}", self.jobs_processed);
    }
}
