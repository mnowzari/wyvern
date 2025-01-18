use crate::{rw_image::ImageDetails, utils::compute_rgb_distance};

use image::{DynamicImage, ImageBuffer, Rgb};

use std::error::Error;

const GREEN_HIGHLIGHT_PX: Rgb<u8> = Rgb([0, 255, 0]);
const BLACKOUT_PX: Rgb<u8> = Rgb([0, 0, 0]);

pub fn edge_detect(
    image_details: &mut ImageDetails,
    threshold: f32,
    blackout: bool,
) -> Result<bool, Box<dyn Error>> {
    // main edge detection function
    let mut image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image_details
        .load_image()
        .expect("Failure loading image!")
        .into_rgb8();

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let mut i: u32 = 1;
    while i < width {
        let mut k: u32 = 1;
        while k < height {
            // reduce each pixel's RGB values in the 2x2 grid to a single value
            let px_top_right: f32 = compute_rgb_distance(image_buf.get_pixel(i - 1, k));
            let px_top_left: f32 = compute_rgb_distance(image_buf.get_pixel(i - 1, k - 1));
            let px_bottom_right: f32 = compute_rgb_distance(image_buf.get_pixel(i, k));
            let px_bottom_left: f32 = compute_rgb_distance(image_buf.get_pixel(i, k - 1));
            // get the mean of all four pixels
            let mean: f32 = (px_top_right + px_top_left + px_bottom_left + px_bottom_right) / 4.0;
            // compute the standard deviation between each pixel in the grid and the mean of the 2x2 grid
            let std_dev: f32 = compute_std_dev(
                mean,
                px_top_right,
                px_top_left,
                px_bottom_left,
                px_bottom_right,
                width,
                height,
            );

            if blackout {
                image_buf.put_pixel(i - 1, k, BLACKOUT_PX);
                image_buf.put_pixel(i - 1, k - 1, BLACKOUT_PX);
                image_buf.put_pixel(i, k, BLACKOUT_PX);
                image_buf.put_pixel(i, k - 1, BLACKOUT_PX);
            }

            // compare the resultant distance to the threshold
            if std_dev > threshold {
                image_buf.put_pixel(i, k, GREEN_HIGHLIGHT_PX);
            }

            k += 2;
        }
        i += 2;
    }

    image_details.save_image(DynamicImage::ImageRgb8(image_buf), "edges")
}

fn compute_std_dev(
    mean: f32,
    px_top_right: f32,
    px_top_left: f32,
    px_bottom_left: f32,
    px_bottom_right: f32,
    width: u32,
    height: u32,
) -> f32 {
    f32::sqrt(
        f32::powf(px_top_right - mean, 2.0)
            + f32::powf(px_top_left - mean, 2.0)
            + f32::powf(px_bottom_left - mean, 2.0)
            + f32::powf(px_bottom_right - mean, 2.0) / (width as f32 * height as f32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_std_dev() {
        let res: f32 = compute_std_dev(35.5324, 44.4, 57.6, 33.2, 19.1, 1920, 1080);

        assert_eq!(23.896727, res);
    }
}
