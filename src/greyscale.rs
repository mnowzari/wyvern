use crate::{rw_image::ImageDetails, utils::average_of_single_rgb_pixel};

use image::{DynamicImage, ImageBuffer, Luma, Rgb};

use std::error::Error;

pub fn greyscale_convert(mut image_details: ImageDetails) -> Result<bool, Box<dyn Error>> {
    let image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        image_details.load_image().expect("Failure loading image!");

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;
    // grayscale output buffer
    let mut output_buf: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for row in 0..width {
        for col in 0..height {
            let color_px: &Rgb<u8> = image_buf.get_pixel(row, col);
            let gray_px: Luma<u8> = Luma([average_of_single_rgb_pixel(color_px)]);
            // let gray_px: Luma<u8> = Luma([compute_rgb_distance(color_px) as u8]);
            output_buf.put_pixel(row, col, gray_px);
        }
    }
    // converting a Luma8 back into Rgb8 for saving. This feels a little cringe to me so might revisit later.
    Ok(image_details.save_image(
        DynamicImage::ImageLuma8(output_buf).into_rgb8(),
        &"greyscale",
    )?)
}
