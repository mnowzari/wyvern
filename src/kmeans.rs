use rand::Rng;
use vek::Vec3;
use crate::image_loader;

const NUM_OF_POINTS: usize = 5;
const DATASET_DIV: usize = 6;

pub fn k_means(file_path: String) -> String {
    let image_data = image_loader::load_image(&file_path)
        .expect("Failed loading image!");

    let image_linear_buf: Vec<Vec3<f32>> = image_data.0;
    let _: u32 = image_data.1; // we don't need to use width and height even though load_image() returns them
    let _: u32 = image_data.2;

    let k_points: [[f32; 3]; NUM_OF_POINTS] = generate_random_points(&image_linear_buf);

    for idx in 1..image_linear_buf.len()/DATASET_DIV {
        let r: f32 = image_linear_buf[idx][0];
        let g: f32 = image_linear_buf[idx][1];
        let b: f32 = image_linear_buf[idx][2];
        // calulcate disance between curr pixel and every kmeans point
        for mut point in k_points {

            let distance = calc_distance(
                &r, &g, &b, 
                &point[0], &point[1], &point[2]);

            if distance < 25.0 {
                point[0] += r;
                point[1] += g;
                point[2] += b;
            }
            else {
                point[0] -= r;
                point[1] -= g;
                point[2] -= b;
            }
        }
    }

    for kpoint in k_points {
        println!("\nR:{}, G:{}, B:{}", kpoint[0], kpoint[1], kpoint[2]);
    }

    file_path
}

fn generate_random_points(image_linear_buf: &Vec<Vec3<f32>>) -> [[f32; 3]; NUM_OF_POINTS] {
    let mut k_points: [[f32; 3]; NUM_OF_POINTS] = [[0.0; 3]; NUM_OF_POINTS];

    for i in 0..k_points.len() {
        let coord = rand::thread_rng().gen_range(100..=image_linear_buf.len()/DATASET_DIV);
        let r: f32 = image_linear_buf[coord][0];
        let g: f32 = image_linear_buf[coord][1];
        let b: f32 = image_linear_buf[coord][2];
        k_points[i] = [r, g, b];
    }
    k_points
}

fn calc_distance(r1: &f32, g1: &f32, b1: &f32, r2: &f32, g2: &f32, b2: &f32) -> f32{
    f32::sqrt(f32::powf(r1-r2, 2.0) + f32::powf(g1-g2, 2.0) + f32::powf(b1-b2, 2.0))
}