use std::error::Error;
use std::ffi::OsString;
use std::path::PathBuf;
use std::ffi::OsStr;

// use clap::builder::OsStr;
use image::io::Reader;

pub struct ImageDetails {
    filepath: OsString, // complete filepath as given by user
    basedir: OsString, // base directory
    filename: OsString, // file name without extension
    extension: OsString, // extension of the given file
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

    pub fn save_image(self, image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,filename_postfix: &str) -> Result<(), Box<dyn Error>>{

        let save_path: OsString = OsString::from(
            format!("{}\\{}_{}.{}", 
                self.basedir.to_str().unwrap(),
                self.filename.to_str().unwrap(),
                filename_postfix,
                self.extension.to_str().unwrap())
        );
        println!("{}", save_path.to_str().unwrap());
        image_buf.save(save_path)?;
        Ok(())
    }

    pub fn get_filename_and_format(file_path: &String) -> ImageDetails {
        let path: PathBuf = PathBuf::from(file_path);
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
