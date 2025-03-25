#[allow(dead_code, unused)]
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use clap::Parser;
use image_processor::ImageProcessor;
use batch::BatchCoordinator;


mod batch;
mod cli;
mod image_processor;
mod threadpool;
mod lib;

fn main() -> Result<(), Box<dyn Error>> {
    // contains our in-memory queue
    let mut batch_coordinator: BatchCoordinator = BatchCoordinator::new()?;
    // Batch coordinator gathers all images and enqueues them
    let arguments = cli::InputArguments::parse();
    batch_coordinator.coordinate(arguments)?;
    // image processor pulls jobs from queue and runs them 
    let mut img_prc: ImageProcessor = ImageProcessor::new()?;
    img_prc.processor_loop(batch_coordinator);
    Ok(())
}
