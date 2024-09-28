use std::{
    fs,
    error::Error,
    path::PathBuf};

use crate::image_downscale::image_downscale;
use crate::rw_image::ImageDetails;
use crate::threadpool::ThreadPool;

use glob::glob;
use num_cpus;

pub fn batch_downscale(directory: String, file_format: String) -> Result<(), Box<dyn Error>> {
    let pool: ThreadPool = ThreadPool::new(num_cpus::get()).unwrap();

    let subdir_name: PathBuf = PathBuf::from("downscaled_images");

    // first, ensure the provided dir is valid
    if !PathBuf::from(&directory).is_dir() {
        panic!("The provided path is not a valid directory!");
    }

    // create search pattern from the given directory and file format
    let pattern: PathBuf = [&directory, &format!( "*.{}", &file_format)].iter().collect();

    println!("Searching {}\n", pattern.to_str().unwrap());

    // glob through directory & downscale each image we encounter and save it in our subdir
    for entry in glob(pattern.to_str().unwrap()).expect("Failed to read directory path!") {
        match entry {
            Ok(image_path) => {
                if check_or_create_subdir(&directory, &subdir_name) {
                    let image_path_string: String = String::from(image_path.to_str().unwrap());

                    let mut img_det_t: ImageDetails = ImageDetails::new_image(&image_path_string);
                    img_det_t.basedir.push(subdir_name.clone());

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

fn check_or_create_subdir(directory: &String, subdir_name: &PathBuf) -> bool {
    let path_to_new_subdir: PathBuf = [directory, subdir_name.to_str().unwrap()].iter().collect();
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
    use std::{fs, env};

    use super::*;

    #[test]
    fn test_check_or_create_subdir_subdir_exists() {
        let cwd: PathBuf = env::current_dir().unwrap().clone();

        let temp_dir: PathBuf = [
            cwd.display().to_string(),
            "temp_check_or_create".to_string()].iter().collect();

        match fs::create_dir(&temp_dir) {
            Ok(_x) => {
                let res = check_or_create_subdir(
                    &cwd.display().to_string(),
                    &temp_dir,
                );
                assert_eq!(true, res);

                // remote temp dir
                let _ = fs::remove_dir(&temp_dir);
                // check again now that the dir doesn't exist
                let res_two: bool = check_or_create_subdir(
                    &cwd.display().to_string(),
                    &temp_dir,
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
