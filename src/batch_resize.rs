// use crate::image_resize;
use std::fs;
use std::path::PathBuf;
use glob::glob;

pub fn batch_resize(directory: String, file_format: String) {
    // main batch resize function
    let pattern: String = format!("{}\\*.{}", directory, file_format);
    println!("Searching {}", pattern);

    // create new sub-directory to store resized images
    let resized_images_base_dir: PathBuf = PathBuf::from(format!("{}\\{}", directory, "resized_images"));
    if resized_images_base_dir.is_dir() {
        let _ = fs::create_dir(resized_images_base_dir);
    }

    // glob through user-provided directory, resize each image we encounter and save it in our subdir
    for entry in glob(pattern.as_str()).expect("Failed to read directory path!") {
        match entry {
            Ok(path) => {
                println!("{}", path.to_str().unwrap());
                // get_filename_from_path(path);
                // image_resize(path, format!("{}\\{}", resized_images_base_dir, ))
            },
            Err(e) => println!("{:?}", e),
        }
    }
}