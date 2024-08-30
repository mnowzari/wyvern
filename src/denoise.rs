use std::error::Error;

use crate::{rw_image::ImageDetails, utils::average_pixel_values};

pub fn denoise(image_details: &mut ImageDetails) -> Result<(), Box<dyn Error>> {
    let mut image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image_details.load_image().expect("Failure loading image!");

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let mut row: u32 = 1;
    while row < width - 1 {
        let mut col: u32 = 1;
        while col < height - 1 {
            let px_top_right: &image::Rgb<u8> = image_buf.get_pixel(row - 1, col);
            let px_top_left: &image::Rgb<u8> = image_buf.get_pixel(row - 1, col - 1);
            let px_bottom_right: &image::Rgb<u8> = image_buf.get_pixel(row, col);
            let px_bottom_left: &image::Rgb<u8> = image_buf.get_pixel(row, col - 1);

            // calculate the average RGB values of the 2x2 grid of pixels
            let px_avg: image::Rgb<u8> =
                average_pixel_values(px_top_right, px_top_left, px_bottom_right, px_bottom_left);

            // calculate distance between each pixel in the grid and the average pixel
            // let distance = calc_distance(
            //     r1,
            //     g1,
            //     b1,
            //     r2,
            //     g2,
            //     b2
            // );

            col += 2
        }
        row += 2
    }
    Ok(())
}
