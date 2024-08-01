use rand::Rng;

use crate::rw_image;

const NUM_OF_POINTS: usize = 20;
const SUBDIV_FOR_FAST_KMEANS: f32 = 3.0;

pub fn k_means_fast(image_file: rw_image::ImageDetails) {
    let image_data: (image::ImageBuffer<image::Rgb<u8>, Vec<u8>>, u32, u32) = image_file.load_image()
        .expect("Failure loading image!");

    let image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image_data.0;
    let width: u32 = image_data.1;
    let height: u32 = image_data.2;
    
    let grid_dim_width: u32 = (width as f32/SUBDIV_FOR_FAST_KMEANS).floor() as u32;
    let grid_dim_height: u32 = (height as f32/SUBDIV_FOR_FAST_KMEANS).floor() as u32;

    let mut grid_results: Vec<[f32; 3]> = vec![];

    let mut i: u32 = grid_dim_height;
    while i < width-1 {
        let mut k: u32 = grid_dim_width;
        while k < height-1 {
            // generate random points in the grid we are at
            let sample_pixels: [[f32; 3]; NUM_OF_POINTS] = generate_random_points_for_grid(
                &image_buf, k, i, grid_dim_width, grid_dim_height);
            
            let mut target_point: [f32; 3] = sample_pixels[0];
            let mut counter: u8 = 1;

            for s_idx in 1..sample_pixels.len() {

                let distance: f32 = calc_distance(
                    &target_point[0], &target_point[1], &target_point[2],
                    &sample_pixels[s_idx][0], &sample_pixels[s_idx][1], &sample_pixels[s_idx][2]
                );
                
                if distance < 10.0 {
                    target_point[0] += sample_pixels[s_idx][0];
                    target_point[1] += sample_pixels[s_idx][1];
                    target_point[2] += sample_pixels[s_idx][2];
                    counter += 1;
                }
            }

            for fin_idx in 0..target_point.len() {
                target_point[fin_idx] = target_point[fin_idx]/counter as f32;
            }
            
            grid_results.push(target_point);

            k += grid_dim_width;
        }
        i += grid_dim_height;
    }

    for grid_color in grid_results {
        println!("R:{} G:{} B:{}", grid_color[0] as u8, grid_color[1] as u8, grid_color[2] as u8);
    }
}

fn generate_random_points_for_grid(image_buf: &image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        k: u32, i: u32, grid_dim_width: u32, grid_dim_height: u32) -> [[f32; 3]; NUM_OF_POINTS] {

    let mut k_points: [[f32; 3]; NUM_OF_POINTS] = [[0.0; 3]; NUM_OF_POINTS];

    for k_idx in 0..NUM_OF_POINTS {
        let rand_y: u32 = rand::thread_rng().gen_range((k-grid_dim_width)..k);
        let rand_x: u32 = rand::thread_rng().gen_range((i-grid_dim_height)..i);

        let temp_px: &image::Rgb<u8> = image_buf.get_pixel(rand_x, rand_y);
        
        k_points[k_idx] = [temp_px[0] as f32, temp_px[1] as f32, temp_px[2] as f32];
    }
    k_points
}

fn calc_distance(r1: &f32, g1: &f32, b1: &f32, r2: &f32, g2: &f32, b2: &f32) -> f32 {
    f32::sqrt(
        f32::powf(r1-r2, 2.0) + 
        f32::powf(g1-g2, 2.0) + 
        f32::powf(b1-b2, 2.0))
}
