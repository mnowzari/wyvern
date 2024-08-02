use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::ffi::OsStr;

use image::io::Reader;

pub struct ImageDetails {
    pub filepath: OsString, // complete filepath
    pub basedir: OsString, // base directory (root)
    pub filename: OsString, // file name without extension
    pub extension: OsString, // extension of the given file
}

impl ImageDetails {

    pub fn load_image(&self) -> Result<(image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, u32, u32), Box<dyn Error>> {
        let rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = Reader::open(&self.filepath)?
            .decode()?
            .into_rgb8();

        let width: u32 = rgb.width();
        let height: u32 = rgb.height();

        Ok((rgb, width, height))
    }

    pub fn save_image(&mut self, image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        filename_postfix: &str) -> Result<(), Box<dyn Error>>{

        let save_path: OsString = OsString::from(
            format!("{}\\{}_{}.{}", 
                self.basedir.to_str().unwrap(),
                self.filename.to_str().unwrap(),
                filename_postfix,
                self.extension.to_str().unwrap())
        );
        // update the filepath field as this struct now represents the 'saved' image
        self.filepath = save_path.clone();

        println!("{}\n", self.filepath
            .to_str()
            .unwrap()
        );
        
        image_buf.save(save_path)?;

        Ok(())
    }
}

pub fn new_image(file_path: &String) -> ImageDetails {
    let path: PathBuf = PathBuf::from(file_path);
    if !path.parent().unwrap().is_dir() {
        panic!("Could not parse the provided directory! Parent dir is not valid.")
    }

    let base_dir: &OsStr = path
        .parent()
        .unwrap()
        .as_os_str();

    let file_ext: &OsStr = match path.extension() {
        Some(x) => x,
        None => &OsStr::new(""),
    };

    let file_name: &OsStr = match path.file_stem() {
        Some(x) => x,
        None => &OsStr::new(""),
    };

    ImageDetails {
        filepath: path.as_os_str().to_os_string(),
        basedir: base_dir.to_os_string(),
        filename: file_name.to_os_string(),
        extension: file_ext.to_os_string(),
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, env};

    use super::*;

    #[test]
    #[should_panic]
    fn test_new_image_bad_path() {
        let _: ImageDetails = new_image(
            &String::from("a\\bad\\path\\"));
    }

    #[test]
    fn test_new_image_valid_path() {
        // create temp dir, as the parent dir must exist
        let cwd: PathBuf = env::current_dir().unwrap().clone();

        let temp_dir: PathBuf = PathBuf::from(
            format!("{}\\temp\\", cwd.display().to_string())
        );
    
        let temp_image_path: PathBuf = PathBuf::from(
            format!("{}\\temp\\fakeimage.png", cwd.display().to_string())
        );

        println!("{}", temp_dir.display().to_string());
        println!("{}", temp_image_path.display().to_string());

        match fs::create_dir(&temp_dir) {
            Ok(_x) => {
                let image_details_instance: ImageDetails = new_image(
                    &String::from(temp_image_path.display().to_string()));

                assert_eq!(image_details_instance.filepath,
                    OsString::from(temp_image_path.display().to_string())
                );
                assert_eq!(image_details_instance.basedir,
                    OsString::from(temp_image_path.parent().unwrap())
                );
                assert_eq!(image_details_instance.filename,
                    OsString::from(temp_image_path.file_stem().unwrap())
                );
                assert_eq!(image_details_instance.extension,
                    OsString::from(temp_image_path.extension().unwrap())
                );
            },
            Err(x) => {
                panic!("\n----\ntest_new_image_valid_path => Problem creating temp dir!\n{x}\n----")
            }
        }
        // cleanup temp dir
        let _ = fs::remove_dir(temp_dir);
    }

    #[test]
    fn test_save_image() {

    }

    #[test]
    fn test_load_image() {

    }
}