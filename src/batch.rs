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
    sync::{Arc, Mutex},
};

use crate::{cli, rw_image::ImageDetails, threadpool::ThreadPool};

// ==========================================
struct BatchJob {
    image_details: ImageDetails,
    arguments: Vec<String>,
    processed: bool,
}

// ==========================================
pub struct BatchCoordinator {
    // job_queue: VecDeque<BatchJob>,
    pub job_queue: Arc<Mutex<VecDeque<BatchJob>>>,
    pub thread_pool: ThreadPool,
}

impl BatchCoordinator {
    pub fn new(number_of_threads: usize) -> Result<Self, Box<dyn Error>> {
        Ok(BatchCoordinator {
            job_queue: Arc::new(Mutex::new(VecDeque::new())),
            thread_pool: ThreadPool::new(number_of_threads)
                .expect("There was an issue starting the thread pool!"),
        })
    }

    fn get_path_from_args(&mut self, args: cli::InputArguments) -> Vec<String> {
        let mut arglist: Vec<String> = vec![]; // maybe this could be a hashmap?
        match args.command {
            cli::ImageCommand::EdgeDetect {
                path,
                threshold,
                blackout,
            } => {
                arglist.push(path.unwrap());
                arglist.push(threshold.to_string());
                arglist.push(blackout.to_string());
            }
            cli::ImageCommand::BatchDownscale { path, extension } => {
                arglist.push(path.unwrap());
                arglist.push(extension.unwrap());
            }
            cli::ImageCommand::PixelSort {
                path,
                threshold,
                direction,
            } => {
                arglist.push(path.unwrap());
                arglist.push(threshold.to_string());
                arglist.push(direction.unwrap().to_string());
            }
            cli::ImageCommand::Denoise {
                path,
                threshold,
                highlight,
            } => {
                arglist.push(path.unwrap());
                arglist.push(threshold.to_string());
                arglist.push(highlight.to_string());
            }
            cli::ImageCommand::Greyscale { path } => {
                arglist.push(path.unwrap());
            }
            cli::ImageCommand::Downscale { path } => {
                arglist.push(path.unwrap());
            }
            cli::ImageCommand::CommonColors { path } => {
                arglist.push(path.unwrap());
            }
        }
        arglist
    }

    pub fn coordinate(&mut self, arguments: cli::InputArguments) -> Result<(), Box<dyn Error>> {
        let list_of_arg_strings: Vec<String> = self.get_path_from_args(arguments);
        let path_from_cli: &String = &list_of_arg_strings[0];

        let path = PathBuf::from(path_from_cli).canonicalize().unwrap();

        // let's do a basic implementation for now
        // we can make this fancy later
        if path.is_file() {
            self.gather_and_queue_images(path.as_os_str(), None, list_of_arg_strings)?;
        } else if path.is_dir() {
            let base_dir: &OsStr = path.as_os_str();
            let file_ext: Option<&OsStr> = Some(&OsStr::new("jpg"));
            // pass arguments along to gather_and_queue_images() too!
            self.gather_and_queue_images(base_dir, file_ext, list_of_arg_strings)
                .expect("Error during the gather and queue step!");
        } else {
            panic!("No file name or extension could be found in the provided path!");
        }
        Ok(())
    }

    pub fn gather_and_queue_images(
        &mut self,
        directory: &OsStr,
        file_format: Option<&OsStr>,
        arguments: Vec<String>,
    ) -> Result<(), Box<dyn Error>> {
        // ensure the provided dir is valid
        if !PathBuf::from(&directory).exists() {
            panic!("The provided path is not valid!");
        }

        let mut glob_pattern: OsString = OsString::from("*.");
        let pattern_components: Vec<&OsStr>;
        match file_format {
            // if this is not a single image, but a path to a directory
            Some(extension) => {
                // create glob pattern
                glob_pattern.push(extension);
                pattern_components = vec![&directory, &glob_pattern];
            }
            None => pattern_components = vec![&directory],
        }
        let pattern: PathBuf = pattern_components.iter().collect();
        println!("Searching {}\n", pattern.to_str().unwrap());

        // secure reference to the in-memory queue
        let jq_arc_ref: Arc<Mutex<VecDeque<BatchJob>>> = self.job_queue.clone();

        // glob through directory & enqueue each image we encounter
        for entry in glob(pattern.to_str().unwrap()).expect("Failed to read directory path!") {
            match entry {
                Ok(image_path) => {
                    // enqueue a new BatchJob
                    jq_arc_ref.lock().unwrap().push_back(BatchJob {
                        image_details: ImageDetails::new_image(&String::from(
                            image_path.to_str().unwrap(),
                        )),
                        arguments: arguments.clone(),
                        processed: false,
                    });
                }
                Err(e) => println!("{:?}", e),
            }
        }
        println!("Size of queue: {}", jq_arc_ref.lock().unwrap().len());
        Ok(())
    }
}
