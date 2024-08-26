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

pub fn calc_distance(r1: &f32, g1: &f32, b1: &f32, r2: &f32, g2: &f32, b2: &f32) -> f32 {
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
