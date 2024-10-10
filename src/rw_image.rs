use std::{error::Error, ffi::OsStr, path::PathBuf};

use image::{io::Reader, ImageBuffer, Rgb};

pub struct ImageDetails {
    pub filepath: PathBuf,  // complete filepath
    pub basedir: PathBuf,   // base directory (root)
    pub filename: PathBuf,  // file name without extension
    pub extension: PathBuf, // extension of the given file
    pub width: u32,
    pub height: u32,
}

impl ImageDetails {
    pub fn load_image(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
        let rgb: ImageBuffer<Rgb<u8>, Vec<u8>> =
            Reader::open(&self.filepath)?.decode()?.into_rgb8();

        self.width = rgb.width();
        self.height = rgb.height();

        Ok(rgb)
    }

    /// When an image is saved, an ImageDetails instance will be updated
    /// represent the 'saved' image. This is why save_image updates
    /// the filepath, width and height fields as well.
    pub fn save_image(
        &mut self,
        image_buf: ImageBuffer<Rgb<u8>, Vec<u8>>,
        filename_postfix: &str,
    ) -> Result<bool, Box<dyn Error>> {
        let filename: PathBuf = PathBuf::from(format!(
            "{}_{}.{}",
            self.filename.to_str().unwrap(),
            filename_postfix,
            self.extension.to_str().unwrap()
        ));

        let mut save_path: PathBuf = PathBuf::from(format!("{}", self.basedir.to_str().unwrap()));
        save_path.push(filename);

        self.filepath = save_path.clone();
        self.width = image_buf.width();
        self.height = image_buf.height();

        println!("{}", self.filepath.to_str().unwrap());

        match image_buf.save(save_path) {
            Ok(_) => Ok(true),
            Err(_) => panic!("An issue occurred during the saving of the image buffer!"),
        }
    }

    pub fn new_image(file_path: &String) -> ImageDetails {
        let path: PathBuf = PathBuf::from(file_path).canonicalize()
            .unwrap();

        if !&path.parent().unwrap().is_dir() {
            panic!("Could not parse the provided directory! Directory is not valid.")
        }

        let base_dir: &OsStr = path.parent().unwrap().as_os_str();

        let file_ext: &OsStr = match path.extension() {
            Some(x) => x,
            None => &OsStr::new(""),
        };

        let file_name: &OsStr = match path.file_stem() {
            Some(x) => x,
            None => &OsStr::new(""),
        };

        ImageDetails {
            filepath: PathBuf::from(path.clone()),
            basedir: PathBuf::from(base_dir),
            filename: PathBuf::from(file_name),
            extension: PathBuf::from(file_ext),
            width: 0,
            height: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{env, ffi::OsString, fs};

    use super::*;

    #[test]
    #[should_panic]
    fn test_new_image_bad_path() {
        let _: ImageDetails = ImageDetails::new_image(&String::from("a\\bad\\path\\"));
    }

    #[test]
    fn test_new_image_valid_path() {
        // create temp dir, as the parent dir must exist
        let cwd: PathBuf = env::current_dir().unwrap().clone();

        let temp_dir: PathBuf = [cwd.display().to_string(), "temp".to_string()]
            .iter()
            .collect();

        let temp_image_path: PathBuf = [
            cwd.display().to_string(),
            "temp".to_string(),
            "fakeimage.png".to_string(),
        ]
        .iter()
        .collect();

        match fs::create_dir(&temp_dir) {
            Ok(_x) => {
                let image_details_instance: ImageDetails =
                    ImageDetails::new_image(&String::from(temp_image_path.display().to_string()));

                assert_eq!(
                    image_details_instance.filepath,
                    OsString::from(temp_image_path.display().to_string())
                );
                assert_eq!(
                    image_details_instance.basedir,
                    OsString::from(temp_image_path.parent().unwrap())
                );
                assert_eq!(
                    image_details_instance.filename,
                    OsString::from(temp_image_path.file_stem().unwrap())
                );
                assert_eq!(
                    image_details_instance.extension,
                    OsString::from(temp_image_path.extension().unwrap())
                );
            }
            Err(x) => {
                panic!("\n----\ntest_new_image_valid_path => Problem creating temp dir!\n{x}\n----")
            }
        }
        // cleanup temp dir
        let _ = fs::remove_dir(temp_dir);
    }
}
