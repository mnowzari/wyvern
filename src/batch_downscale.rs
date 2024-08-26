use std::{error::Error, ffi::OsString, fs, path::PathBuf};

use glob::glob;
use num_cpus;

use crate::image_downscale::image_downscale;
use crate::rw_image::{self, ImageDetails};
use crate::threadpool::ThreadPool;

pub fn batch_downscale(directory: String, file_format: String) -> Result<(), Box<dyn Error>> {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()).unwrap();

    let subdir_name: String = String::from("downscaled_images");

    // first, ensure the provided dir is valid
    if !PathBuf::from(&directory).is_dir() {
        panic!("The provided path is not a valid directory!");
    }

    // create search pattern from the given directory and file format
    let pattern: String = format!("{}\\*.{}", directory, file_format);
    println!("Searching {}\n", pattern);

    // glob through directory & downscale each image we encounter and save it in our subdir
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
                        match image_downscale(&mut img_det_t) {
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

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn test_check_or_create_subdir_subdir_exists() {
        let cwd: PathBuf = env::current_dir().unwrap().clone();

        let temp_dir: PathBuf = PathBuf::from(format!(
            "{}\\temp_check_or_create\\",
            cwd.display().to_string()
        ));

        match fs::create_dir(&temp_dir) {
            Ok(_x) => {
                let res = check_or_create_subdir(
                    &cwd.display().to_string(),
                    &temp_dir.display().to_string(),
                );
                assert_eq!(true, res);

                // remote temp dir
                let _ = fs::remove_dir(&temp_dir);
                // check again now that the dir doesn't exist
                let res_two: bool = check_or_create_subdir(
                    &cwd.display().to_string(),
                    &temp_dir.display().to_string(),
                );
                assert_eq!(true, res_two);
            }
            Err(x) => {
                panic!(
                    "\n----\ntest_check_or_create_subdir_subdir_exists => Problem creating temp dir!\n{x}\n----"
                )
            }
        }
        // cleanup temp dir
        let _ = fs::remove_dir(&temp_dir);
    }
}
