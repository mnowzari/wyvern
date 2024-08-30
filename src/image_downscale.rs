use crate::{rw_image::ImageDetails, utils::average_pixel_values};

use image::{ImageBuffer, Rgb};

use std::error::Error;

const DOWNSCALE_FACTOR: u32 = 2;

pub fn image_downscale(image_details: &mut ImageDetails) -> Result<bool, Box<dyn Error>> {
    // main resizing function
    let image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        image_details.load_image().expect("Failure loading image!");

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let mut output_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(width / DOWNSCALE_FACTOR, height / DOWNSCALE_FACTOR);

    let mut i: u32 = 1;
    while i < width - 1 {
        let mut k: u32 = 1;
        while k < height - 1 {
            let px_top_right: &Rgb<u8> = image_buf.get_pixel(i - 1, k);
            let px_top_left: &Rgb<u8> = image_buf.get_pixel(i - 1, k - 1);
            let px_bottom_right: &Rgb<u8> = image_buf.get_pixel(i, k);
            let px_bottom_left: &Rgb<u8> = image_buf.get_pixel(i, k - 1);

            // calculate the average RGB values of the 2x2 grid of pixels
            let px_avg: Rgb<u8> =
                average_pixel_values(px_top_right, px_top_left, px_bottom_right, px_bottom_left);
            output_buf.put_pixel(i / DOWNSCALE_FACTOR, k / DOWNSCALE_FACTOR, px_avg);
            k += 2;
        }
        i += 2;
    }

    Ok(image_details.save_image(output_buf, &"downscaled")?)
}
