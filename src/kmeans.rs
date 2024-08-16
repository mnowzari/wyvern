use rand::Rng;
use std::error::Error;

use crate::rw_image;

const NUM_OF_POINTS: usize = 16;
const SUBDIV_FOR_FAST_KMEANS: f32 = 3.0;
const DISTANCE: f32 = 10.0;

pub fn k_means_fast(mut image_details: rw_image::ImageDetails) -> Result<(), Box<dyn Error>> {
    let image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image_details.load_image().expect("Failure loading image!");

    let width: u32 = image_details.width;
    let height: u32 = image_details.height;

    let grid_dim_width: u32 = (width as f32 / SUBDIV_FOR_FAST_KMEANS).floor() as u32;
    let grid_dim_height: u32 = (height as f32 / SUBDIV_FOR_FAST_KMEANS).floor() as u32;

    let mut grid_results: Vec<[f32; 3]> = vec![];

    let mut row: u32 = grid_dim_height;
    while row < width - 1 {
        let mut col: u32 = grid_dim_width;
        while col < height - 1 {
            // generate random points in the grid we are at
            let sample_pixels: [[f32; 3]; NUM_OF_POINTS] = generate_random_points_for_grid(
                &image_buf,
                row,
                col,
                grid_dim_width,
                grid_dim_height,
            );

            let mut target_point: [f32; 3] = sample_pixels[0];
            let mut counter: u8 = 1;

            for s_idx in 1..sample_pixels.len() {
                let distance: f32 = calc_distance(
                    &target_point[0],
                    &target_point[1],
                    &target_point[2],
                    &sample_pixels[s_idx][0],
                    &sample_pixels[s_idx][1],
                    &sample_pixels[s_idx][2],
                );

                if distance < DISTANCE {
                    target_point[0] += sample_pixels[s_idx][0];
                    target_point[1] += sample_pixels[s_idx][1];
                    target_point[2] += sample_pixels[s_idx][2];
                    counter += 1;
                }
            }

            for fin_idx in 0..target_point.len() {
                target_point[fin_idx] = target_point[fin_idx] / counter as f32;
            }

            grid_results.push(target_point);

            col += grid_dim_width;
        }
        row += grid_dim_height;
    }

    for grid_color in grid_results {
        println!(
            "R:{} G:{} B:{}",
            grid_color[0] as u8, grid_color[1] as u8, grid_color[2] as u8
        );
    }
    Ok(())
}

fn generate_random_points_for_grid(
    image_buf: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    row: u32,
    col: u32,
    grid_dim_width: u32,
    grid_dim_height: u32,
) -> [[f32; 3]; NUM_OF_POINTS] {
    let mut k_points: [[f32; 3]; NUM_OF_POINTS] = [[0.0; 3]; NUM_OF_POINTS];

    for k_idx in 0..NUM_OF_POINTS {
        let rand_x: u32 = rand::thread_rng().gen_range((row - grid_dim_height)..row);
        let rand_y: u32 = rand::thread_rng().gen_range((col - grid_dim_width)..col);

        let temp_px: &image::Rgb<u8> = image_buf.get_pixel(rand_x, rand_y);

        k_points[k_idx] = [temp_px[0] as f32, temp_px[1] as f32, temp_px[2] as f32];
    }
    k_points
}

fn calc_distance(r1: &f32, g1: &f32, b1: &f32, r2: &f32, g2: &f32, b2: &f32) -> f32 {
    f32::sqrt(f32::powf(r1 - r2, 2.0) + f32::powf(g1 - g2, 2.0) + f32::powf(b1 - b2, 2.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_distance() {
        let expected: [f32; 5] = [173.20508, 5.0990195, 0.0, 118.54113, 80.80842];

        let test_points: [[f32; 6]; 5] = [
            [0.0, 0.0, 0.0, 100.0, 100.0, 100.0],
            [10.0, 15.0, 13.0, 14.0, 14.0, 10.0],
            [33.0, 45.0, 67.0, 33.0, 45.0, 67.0],
            [101.0, -34.0, 59.0, -9.0, -78.0, 55.0],
            [55.0, 3.0, 1.0, 88.0, 74.0, 21.0],
        ];

        for idx in 0..5 {
            let res: f32 = calc_distance(
                &test_points[idx][0],
                &test_points[idx][1],
                &test_points[idx][2],
                &test_points[idx][3],
                &test_points[idx][4],
                &test_points[idx][5],
            );

            assert_eq!(expected[idx], res);
        }
    }
}
