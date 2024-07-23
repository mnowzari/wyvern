use image::io::Reader;
use vek::Vec3;
use std::error::Error;

pub fn load_image(file_path: &String) -> Result<(Vec<Vec3<f32>>, u32, u32), Box<dyn Error>> {
    let img = Reader::open(file_path)?.decode()?;

    let rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = img.into_rgb8();
    let mut linear: Vec<Vec3<f32>> = vec![Vec3::<f32>::zero(); rgb.as_raw().len()];

    let width: u32 = rgb.width();
    let height: u32 = rgb.height();

    // zip the pixels into the linear buffer we created above
    rgb.pixels()
        .zip(linear.iter_mut())
        .for_each(|(rgb, linear)| {
            let rgbvec = Vec3::<u8>::from(rgb.0);
            // *linear = rgbvec.numcast::<f32>().unwrap().map(|x| x / 255.0);
            *linear = rgbvec.numcast::<f32>().unwrap();
        });
    Ok((linear, width, height))
}

pub fn save_image(image: Vec<Vec3<f32>>, width: u32, height: u32, file_path: String) -> Result<(), Box<dyn Error>>{
    let mut rgb: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);
    // zip the output back into the img rgb
    image
        .into_iter()
            .zip(rgb.pixels_mut())
            .for_each(|(linear, rgb)| {
                // let transformed = (linear * 255.0).clamped(0.0, 255.0);
                rgb.0 = linear.numcast().unwrap().into_array();
            });

    println!("./{file_path}_modified.png");
    rgb.save(format!("{file_path}_modified.png"))?;
    Ok(())
}