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

        let base_dir: &OsStr = path.parent().unwrap().as_os_str();
        let mut file_name: Option<&OsStr> = None;
        let file_ext: &OsStr = match path.extension() {
            Some(x) => {
                file_name = path.file_name(); // grab filename if this is a dir
                x
            },
            None => OsStr::new(""),
        };

        // pass arguments along to gather_and_queue_images() too!

        Ok(self.gather_and_queue_images(base_dir, file_ext, file_name)?)
    }

    pub fn gather_and_queue_images(
        &mut self,
        directory: &OsStr,
        file_format: &OsStr,
        file_name: Option<&OsStr>,
    ) -> Result<(), Box<dyn Error>> {
    
        // ensure the provided dir is valid
        if !PathBuf::from(&directory).is_dir() {
            panic!("The provided path is not a valid directory!");
        }

        // create glob pattern
        let wildcard: &OsString = &OsString::from("*.");
        let pattern_components: Vec<&OsStr>;
        match file_name {
            Some(filename) => {
                pattern_components = vec![&directory, &filename];
            }
            None => pattern_components = vec![&directory, &wildcard, &file_format],
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
                    println!("Size of queue: {}", self.job_queue.len());
                }
                Err(e) => println!("{:?}", e),
            }
        }
        Ok(())
    }
}
