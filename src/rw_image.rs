use image::io::Reader;
use vek::Vec3;
use std::error::Error;

pub fn load_image(file_path: &String) -> Result<(image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, u32, u32),
        Box<dyn Error>> {
    // let img = Reader::open(file_path)?.decode()?;
    let rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = Reader::open(file_path)?
        .decode()?
        .into_rgb8();
    let width: u32 = rgb.width();
    let height: u32 = rgb.height();
    Ok((rgb, width, height))
}

pub fn load_image_flattened(file_path: &String) -> Result<(Vec<Vec3<f32>>, u32, u32), Box<dyn Error>> {
    let img = Reader::open(file_path)?.decode()?;

    let rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = img.into_rgb8();
    let width: u32 = rgb.width();
    let height: u32 = rgb.height();

    let mut linear: Vec<Vec3<f32>> = vec![Vec3::<f32>::zero(); (width*height) as usize];

    // zip the pixels into the linear buffer we created above
    rgb.pixels()
        .zip(linear.iter_mut())
        .for_each(|(rgb, linear)| {
            let rgbvec = Vec3::<u8>::from(rgb.0);
            *linear = rgbvec.numcast::<f32>().unwrap();
        });
    Ok((linear, width, height))
}

pub fn save_image(image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        width: u32, height: u32, file_path: &String, filename_postfix: &String) -> Result<(), Box<dyn Error>>{

    let filename: &str = get_filename_from_filepath(file_path);
    println!("./{filename}_{filename_postfix}.png");
    image.save(format!("{filename}_{filename_postfix}.png"))?;
    Ok(())
}

pub fn save_image_flattened(image: Vec<Vec3<u8>>,
        width: u32, height: u32, file_path: &String, filename_postfix: &String) -> Result<(), Box<dyn Error>>{

    let mut rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);
    // zip the input image linear buffer back into rgb
    image
        .into_iter()
            .zip(rgb.pixels_mut())
            .for_each(|(linear, rgb)| {
                // let transformed = (linear * 255.0).clamped(0.0, 255.0);
                rgb.0 = linear.numcast().unwrap().into_array();
            });

    let filename: &str = get_filename_from_filepath(file_path);
    println!("./{filename}_{filename_postfix}.png");
    rgb.save(format!("{file_path}_{filename_postfix}.png"))?;
    Ok(())
}

fn get_filename_from_filepath(file_path: &String) -> &str {
    let filename_vec: Vec<&str> = file_path.split(".png").collect();
    filename_vec[0]
}