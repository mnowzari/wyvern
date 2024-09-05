///
/// This utils.rs file holds utility functions that
/// are used in two or more locations.
/// If a function is only used only in a single location,
/// then it should should not live here.
///
/// I think it is more helpful to have helper methods live near
/// where they are 'used', but if that helper method is
/// general enough to have utility in other parts of the code,
/// then it should live in utils.rs
///

pub fn compute_rgb_distance(pixel: &image::Rgb<u8>) -> f32 {
    f32::sqrt(
        f32::powf(pixel[0] as f32, 2.0)
            + f32::powf(pixel[1] as f32, 2.0)
            + f32::powf(pixel[2] as f32, 2.0),
    )
}

pub fn calc_distance(r1: &f32, g1: &f32, b1: &f32, r2: &f32, g2: &f32, b2: &f32) -> f32 {
    f32::sqrt(f32::powf(r1 - r2, 2.0) + f32::powf(g1 - g2, 2.0) + f32::powf(b1 - b2, 2.0))
}

pub fn average_of_single_rgb_pixel(pixel: &image::Rgb<u8>) -> u8 {
    ((pixel[0] as f32 + pixel[1] as f32 + pixel[2] as f32) / 3.0) as u8
}

pub fn average_pixel_values(
    top_right_pixel: &image::Rgb<u8>,
    top_left_pixel: &image::Rgb<u8>,
    bottom_right_pixel: &image::Rgb<u8>,
    bottom_left_pixel: &image::Rgb<u8>,
) -> image::Rgb<u8> {
    let red_avg: u32 = (top_right_pixel[0] as u32
        + top_left_pixel[0] as u32
        + bottom_right_pixel[0] as u32
        + bottom_left_pixel[0] as u32)
        / 4;
    let gre_avg: u32 = (top_right_pixel[1] as u32
        + top_left_pixel[1] as u32
        + bottom_right_pixel[1] as u32
        + bottom_left_pixel[1] as u32)
        / 4;
    let blu_avg: u32 = (top_right_pixel[2] as u32
        + top_left_pixel[2] as u32
        + bottom_right_pixel[2] as u32
        + bottom_left_pixel[2] as u32)
        / 4;
    image::Rgb([red_avg as u8, gre_avg as u8, blu_avg as u8])
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use super::*;

    #[test]
    fn test_average_of_single_rgb_pixel() {
        let test_pixels: [Rgb<u8>; 4] = [
            Rgb([0, 0, 0]),
            Rgb([55, 55, 55]),
            Rgb([24, 68, 178]),
            Rgb([255, 255, 255]),
        ];

        let expected: [u8; 4] = [0, 55, 90, 255];

        for i in 0..test_pixels.len() {
            assert_eq!(average_of_single_rgb_pixel(&test_pixels[i]), expected[i]);
        }
    }

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

    #[test]
    fn test_average_pixel_values() {
        let expected: [image::Rgb<u8>; 3] = [
            image::Rgb([5, 5, 5]),
            image::Rgb([15, 18, 44]),
            image::Rgb([13, 10, 4]),
        ];

        let test_points: [[image::Rgb<u8>; 4]; 3] = [
            [
                image::Rgb([10, 0, 0]),
                image::Rgb([0, 10, 0]),
                image::Rgb([0, 0, 10]),
                image::Rgb([10, 10, 10]),
            ],
            [
                image::Rgb([10, 15, 13]),
                image::Rgb([14, 14, 10]),
                image::Rgb([33, 45, 67]),
                image::Rgb([3, 1, 88]),
            ],
            [
                image::Rgb([0, 30, 0]),
                image::Rgb([45, 0, 0]),
                image::Rgb([0, 0, 17]),
                image::Rgb([9, 12, 0]),
            ],
        ];

        for idx in 0..3 {
            let res: image::Rgb<u8> = average_pixel_values(
                &test_points[idx][0],
                &test_points[idx][1],
                &test_points[idx][2],
                &test_points[idx][3],
            );

            assert_eq!(expected[idx], res);
        }
    }

    #[test]
    fn test_compute_rgb_distance() {
        let expected: [f32; 5] = [0.0, 22.22611, 87.19518, 121.81133, 55.090836];

        let test_points: [image::Rgb<u8>; 5] = [
            image::Rgb([0, 0, 0]),
            image::Rgb([10, 15, 13]),
            image::Rgb([33, 45, 67]),
            image::Rgb([101, 34, 59]),
            image::Rgb([55, 3, 1]),
        ];

        for idx in 0..5 {
            let res: f32 = compute_rgb_distance(&test_points[idx]);
            assert_eq!(expected[idx], res);
        }
    }
}
