use crate::{cli::PixelSortDir, rw_image::ImageDetails, utils::calc_distance};

use image::{DynamicImage, ImageBuffer, Rgb};

use rand::Rng;
use std::error::Error;

pub fn pixel_sort(
    image_details: &mut ImageDetails,
    threshold: f32,
    direction: PixelSortDir,
) -> Result<bool, Box<dyn Error>> {
    let mut image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image_details
        .load_image()
        .expect("Failure loading image!")
        .into_rgb8();

    let width = image_details.width;
    let height = image_details.height;

    let random_x_coord: u32 = rand::thread_rng().gen_range(2..width);
    let random_y_coord: u32 = rand::thread_rng().gen_range(2..height);
    // clone this pixel as we will need it later
    let comparison_px: Rgb<u8> = image_buf.get_pixel(random_x_coord, random_y_coord).clone();

    for row in 1..width {
        for col in 1..height {
            let px: Rgb<u8> = image_buf.get_pixel(row, col).clone();

            // calculate distance
            let distance = calc_distance(
                &f32::from(px[0]),
                &f32::from(px[1]),
                &f32::from(px[2]),
                &f32::from(comparison_px[0]),
                &f32::from(comparison_px[1]),
                &f32::from(comparison_px[2]),
            );

            // check distance and swap elements if under threshold
            // if it is, swap the pixels around
            if distance < threshold {
                let mut row_n: u32 = row;
                let mut col_n: u32 = col;
                match direction {
                    PixelSortDir::Horizontal => {
                        row_n -= 1;
                    }
                    PixelSortDir::Vertical => {
                        col_n -= 1;
                    }
                    PixelSortDir::Diagonal => {
                        row_n -= 1;
                        col_n -= 1
                    }
                }
                image_buf.put_pixel(row, col, image_buf.get_pixel(row_n, col_n).clone());
            }
        }
    }
    image_details.save_image(DynamicImage::ImageRgb8(image_buf), "pixelsorted")
}
