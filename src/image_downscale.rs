use crate::{rw_image::ImageDetails, utils::average_pixel_values};

use image::{DynamicImage, ImageBuffer, Rgb};

use std::error::Error;

const DOWNSCALE_FACTOR: u32 = 2;

pub fn image_downscale(image_details: &mut ImageDetails) -> Result<bool, Box<dyn Error>> {
    // main resizing function
    let image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image_details
        .load_image()
        .expect("Failure loading image!")
        .into_rgb8();

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let mut output_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
        ImageBuffer::new(width / DOWNSCALE_FACTOR, height / DOWNSCALE_FACTOR);

    let mut row: u32 = 1;
    while row < width - 1 {
        let mut col: u32 = 1;
        while col < height - 1 {
            let px_top_right: &Rgb<u8> = image_buf.get_pixel(row - 1, col);
            let px_top_left: &Rgb<u8> = image_buf.get_pixel(row - 1, col - 1);
            let px_bottom_right: &Rgb<u8> = image_buf.get_pixel(row, col);
            let px_bottom_left: &Rgb<u8> = image_buf.get_pixel(row, col - 1);

            // calculate the average RGB values of the 2x2 grid of pixels
            let px_avg: Rgb<u8> =
                average_pixel_values(px_top_right, px_top_left, px_bottom_right, px_bottom_left);
            output_buf.put_pixel(row / DOWNSCALE_FACTOR, col / DOWNSCALE_FACTOR, px_avg);
            col += 2;
        }
        row += 2;
    }

    image_details.save_image(DynamicImage::ImageRgb8(output_buf), "downscaled")
}
