use std::{error::Error, ffi::OsString, fs, path::PathBuf};

use glob::glob;

use num_cpus;

use crate::image_resize::image_resize;
use crate::rw_image::{self, ImageDetails};
use crate::threadpool::ThreadPool;

pub fn batch_resize(directory: String, file_format: String) -> Result<(), Box<dyn Error>> {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()).unwrap();

    let subdir_name: String = String::from("resized_images");

    // first, ensure the provided dir is valid
    if !PathBuf::from(&directory).is_dir() {
        panic!("The provided path is not a valid directory!");
    }

    // create search pattern from the given directory and file format
    let pattern: String = format!("{}\\*.{}", directory, file_format);
    println!("Searching {}\n", pattern);

    // glob through directory & resize each image we encounter and save it in our subdir
    for entry in glob(pattern.as_str()).expect("Failed to read directory path!") {
        match entry {
            Ok(image_path) => {
                if check_or_create_subdir(&directory, &subdir_name) {
                    let image_path_string: String = String::from(image_path.to_str().unwrap());

                    let mut img_det_t: ImageDetails =
                        rw_image::ImageDetails::new_image(&image_path_string);

                    img_det_t.basedir = OsString::from(format!(
                        "{}\\{}",
                        img_det_t.basedir.to_str().unwrap(),
                        subdir_name
                    ));

                    pool.execute(move || {
                        match image_resize(&mut img_det_t) {
                            Ok(_x) => {}
                            Err(_x) => println!(
                                "An error occurred resizing {}!",
                                &img_det_t.basedir.to_str().unwrap()
                            ),
                        };
                    });
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(())
}

fn check_or_create_subdir(directory: &String, subdir_name: &String) -> bool {
    let path_to_new_subdir: PathBuf = [directory, subdir_name].iter().collect();
    if !path_to_new_subdir.is_dir() {
        match fs::create_dir(path_to_new_subdir) {
            Ok(_x) => return true,
            Err(_x) => return false,
        }
    }
    true
}
