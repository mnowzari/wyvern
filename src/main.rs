use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use batch::BatchCoordinator;
#[allow(dead_code, unused)]
use clap::Parser;

mod batch;
mod batch_downscale;
mod cli;
mod denoise;
mod edge_detect;
mod greyscale;
mod image_downscale;
mod image_processor;
mod kmeans;
mod pixelsort;
mod rw_image;
mod threadpool;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = cli::InputArguments::parse();

    // Batch coordinator gathers all images and enqueues them
    let mut batch_coordinator: BatchCoordinator = BatchCoordinator::new(num_cpus::get())?;
    batch_coordinator.coordinate(arguments)?;

    // image processor pulls jobs from queue and runs them 
    // Maybe we can start the processor first and have it listen to the queue
    // this way it can start processing the moment it's occupied,
    // not when the coordinator is done

    Ok(())
}
