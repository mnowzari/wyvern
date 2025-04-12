use std::error::Error;

use crate::{
    rw_image::ImageDetails,
    utils::{average_pixel_values, calc_distance},
};

use image::{DynamicImage, ImageBuffer, Rgb};

const GREEN_HIGHLIGHT_PX: Rgb<u8> = Rgb([0, 255, 0]);

pub fn denoise(
    image_details: &mut ImageDetails,
    threshold: f32,
    highlight: bool,
) -> Result<bool, Box<dyn Error>> {
    let mut image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image_details
        .load_image()
        .expect("Failure loading image!")
        .into_rgb8();

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let mut row: u32 = 1;
    while row < width {
        let mut col: u32 = 1;
        while col < height {
            // array of references to pixels in image_buf
            let px_subset: [&Rgb<u8>; 4] = [
                image_buf.get_pixel(row - 1, col),
                image_buf.get_pixel(row - 1, col - 1),
                image_buf.get_pixel(row, col),
                image_buf.get_pixel(row, col - 1),
            ];

            // calculate the average RGB value for the 2x2 grid we are at
            let px_avg: Rgb<u8> =
                average_pixel_values(px_subset[0], px_subset[1], px_subset[2], px_subset[3]);

            let replacement_px: &Rgb<u8> = match highlight {
                false => {
                    &px_avg
                }
                true => {
                    &GREEN_HIGHLIGHT_PX
                }
            };

            if let Some(x) = get_hot_pixel_index(px_subset, &px_avg, threshold) { match x {
                0 => image_buf.put_pixel(row - 1, col, *replacement_px),
                1 => image_buf.put_pixel(row - 1, col - 1, *replacement_px),
                2 => image_buf.put_pixel(row, col, *replacement_px),
                3 => image_buf.put_pixel(row, col - 1, *replacement_px),
                _ => {}
            } }

            col += 2
        }
        row += 2
    }
    image_details.save_image(DynamicImage::ImageRgb8(image_buf), "denoised")
}

fn get_hot_pixel_index(
    pixel_subset: [&Rgb<u8>; 4],
    px_avg: &Rgb<u8>,
    threshold: f32,
) -> Option<usize> {
    let mut max_idx: usize = 0;
    let mut max_dist: f32 = 0.0;
    let mut all_dist: [i32; 4] = [0; 4];

    for i in 0..pixel_subset.len() {
        let dist: f32 = calc_distance(
            &f32::from(px_avg[0]),
            &f32::from(px_avg[1]),
            &f32::from(px_avg[2]),
            &f32::from(pixel_subset[i][0]),
            &f32::from(pixel_subset[i][1]),
            &f32::from(pixel_subset[i][2]),
        );

        if dist > max_dist {
            max_dist = dist;
            max_idx = i;
        }

        all_dist[i] = dist as i32; // bc Rust won't sort an array of f32
    }

    all_dist.sort();

    let typ_dist_sum: i32 = all_dist[0..3].iter().sum();
    let typical_dist: f32 = typ_dist_sum as f32 / 3.0;

    if max_dist > typical_dist * threshold {
        Some(max_idx)
    } else {
        None
    }
}
