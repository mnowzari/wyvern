use crate::{
    rw_image::ImageDetails,
    utils::{average_pixel_values, calc_distance},
};

use image::{DynamicImage, ImageBuffer, Rgb};

use rand::Rng;
use std::error::Error;

const TEST_POINTS: usize = 24;
const SAMPLE_POINTS: usize = 512;
const DISTANCE: f32 = 10.0;

pub fn k_means_fast(image_details: &mut ImageDetails) -> Result<bool, Box<dyn Error>> {
    let image_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = image_details
        .load_image()
        .expect("Failure loading image!")
        .into_rgb8();

    let mut output_buf: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(3072, 512);

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let mut test_points: [[f32; 3]; TEST_POINTS] = [[0.0; 3]; TEST_POINTS];
    let mut test_point_counters: [f32; TEST_POINTS] = [1.0; TEST_POINTS];

    for k_idx in 0..SAMPLE_POINTS {
        let rand_w: u32 = rand::thread_rng().gen_range(128..width);
        let rand_h: u32 = rand::thread_rng().gen_range(128..height);

        // get the average RGB values of a 2x2 grid
        let tr: &Rgb<u8> = image_buf.get_pixel(rand_w - 1, rand_h);
        let tl: &Rgb<u8> = image_buf.get_pixel(rand_w - 1, rand_h - 1);
        let br: &Rgb<u8> = image_buf.get_pixel(rand_w, rand_h);
        let bl: &Rgb<u8> = image_buf.get_pixel(rand_w, rand_h - 1);

        let temp_px: Rgb<u8> = average_pixel_values(tr, tl, br, bl);

        if k_idx < TEST_POINTS {
            // the first several points we generate become our test points
            test_points[k_idx] = [temp_px[0] as f32, temp_px[1] as f32, temp_px[2] as f32];
        } else {
            // the rest become sample points that we can compare against the test points
            for (tp_idx, tp) in test_points.iter_mut().enumerate() {

                let distance: f32 = calc_distance(
                    &(temp_px[0] as f32),
                    &(temp_px[1] as f32),
                    &(temp_px[2] as f32),
                    &tp[0],
                    &tp[1],
                    &tp[2],
                );

                if distance < DISTANCE {
                    // increment this test points' counter
                    test_point_counters[tp_idx] += 1.0;
                    // and calculate its new average
                    tp[0] = (tp[0] + temp_px[0] as f32) / test_point_counters[tp_idx];
                    tp[1] = (tp[1] + temp_px[1] as f32) / test_point_counters[tp_idx];
                    tp[2] = (tp[2] + temp_px[2] as f32) / test_point_counters[tp_idx];
                    break;
                }
            }
        }
    }

    // generate the output image
    let mut start_x: u32 = 0;
    let mut end_x: u32 = 128;
    for grid_color in test_points {
        for x in start_x..end_x {
            for y in 0..512 {
                output_buf.put_pixel(
                    x,
                    y,
                    Rgb([
                        grid_color[0] as u8,
                        grid_color[1] as u8,
                        grid_color[2] as u8,
                    ]),
                );
            }
        }
        start_x += 128;
        end_x += 128;
    }

    // create a new ImageDetails object to create the output image
    let mut output_image: ImageDetails =
        ImageDetails::new_image(&image_details.filepath.display().to_string());
    output_image.save_image(DynamicImage::ImageRgb8(output_buf), "common_colors")
}
