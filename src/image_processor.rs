use std::error::Error;

use crate::batch::BatchCoordinator;

pub struct ImageProcessor {
    pub jobs_processed: u32,
}

impl ImageProcessor {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(ImageProcessor {
            jobs_processed: 0,
        })
    }

    fn processor_loop(coordinator: BatchCoordinator) {
        // batch coordinator has the thread pool
        // as well as the job queue
        while coordinator.job_queue.lock().unwrap().pop_front().unwrap() {

        }
    }
}
