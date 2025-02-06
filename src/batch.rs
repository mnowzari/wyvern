// This is a batch job runner that can perform batch image processing
// of any type, not just downscaling
// This will be in charge of coordinating the files that need jobs ran against, and the actual
// execution of the jobs

// Flows
// a. wyvern is given a path to a directoryand and a file format to filter by (png, jpg, etc)
// b. wyvern is given a list of individual images (blegh)
// I like option a. better, let us assume that use case!

// I think if we wanted to implement a run trait on each type of processing,
// then each processing type (denoise, edge detect, etc) would need to be turned
// into a struct that implements a concrete implementation of a trait

// Another idea - we can use ImageDetails to load images into memory during the gathering phase
// to front-load the read-in IO and have the batch processing ONLY worry about actually performing
// the processing

// A batch job can have 2 states - processed or non-processed. Once a batch job is flagged as 'processed'
// it can get enqueued again for saving to disk

// if the provided path is a directory, do <>
// else if the provided path is an image, do <>

use glob::glob;
use std::{
    collections::VecDeque,
    error::Error,
    ffi::{OsStr, OsString},
    path::PathBuf,
};

use crate::{rw_image::ImageDetails, threadpool::ThreadPool};

// ==========================================
struct BatchJob {
    image_details: ImageDetails,
    arguments: Vec<String>,
    processed: bool,
}

impl BatchJob {
    pub fn run() {}
}

// ==========================================
pub struct BatchCoordinator {
    job_queue: VecDeque<BatchJob>,
    thread_pool: ThreadPool,
}

impl BatchCoordinator {
    pub fn new(number_of_threads: usize) -> Result<Self, Box<dyn Error>> {
        Ok(BatchCoordinator {
            job_queue: VecDeque::new(),
            thread_pool: ThreadPool::new(number_of_threads)
                .expect("There was an issue starting the thread pool!"),
        })
    }

    pub fn coordinate(
        &mut self,
        path_from_cli: Option<String>,
        arguments: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {

        let path = PathBuf::from(path_from_cli.unwrap())
            .canonicalize()
            .unwrap();
        
        // let's do a basic implementation for now
        // we can make this fancy later
        if path.is_file() {
            self.gather_and_queue_images(path.as_os_str(), None)?;
        }
        else if path.is_dir() {
            let base_dir: &OsStr = path.as_os_str();
            let file_ext: Option<&OsStr> = Some(&OsStr::new("jpg"));
            // pass arguments along to gather_and_queue_images() too!
            self.gather_and_queue_images(base_dir, file_ext).expect(
                "Error during the gather and queue step!"
            );
        }
        else {
            panic!("No file name or extension could be found in the provided path!");
        }
        Ok(())
    }

    pub fn gather_and_queue_images(
        &mut self,
        directory: &OsStr,
        file_format: Option<&OsStr>,
    ) -> Result<(), Box<dyn Error>> {
    
        // ensure the provided dir is valid
        if !PathBuf::from(&directory).exists() {
            panic!("The provided path is not valid!");
        }

        let mut glob_pattern: OsString = OsString::from("*.");
        let pattern_components: Vec<&OsStr>;
        match file_format { // if this is not a single image, but a path to a directory
            Some(extension) => {
                // create glob pattern
                glob_pattern.push(extension);
                pattern_components = vec![&directory, &glob_pattern];
            }
            None => pattern_components = vec![&directory]
        }
        let pattern: PathBuf = pattern_components.iter().collect();
        println!("Searching {}\n", pattern.to_str().unwrap());

        // glob through directory & enqueue each image we encounter
        for entry in glob(pattern.to_str().unwrap()).expect("Failed to read directory path!") {
            match entry {
                Ok(image_path) => {
                    // enqueue a new BatchJob
                    self.job_queue.push_back(BatchJob {
                        image_details: ImageDetails::new_image(&String::from(
                            image_path.to_str().unwrap(),
                        )),
                        arguments: Vec::new(),
                        processed: false,
                    });
                }
                Err(e) => println!("{:?}", e),
            }
        }
        println!("Size of queue: {}", self.job_queue.len());
        Ok(())
    }
}
