use crate::rw_image::ImageFileDetails;

const GREEN_HIGHLIGHT_PX: image::Rgb<u8> = image::Rgb([0, 255, 0]);
const BLACKOUT_PX: image::Rgb<u8> = image::Rgb([0, 0, 0]);

pub fn edge_detect(image_file: ImageFileDetails, threshold: f32, blackout: bool) {
    // main edge detection function
    // let image_data = rw_image::load_image(&file_path)
    //     .expect("Failed loading image!");
    let image_data: (image::ImageBuffer<image::Rgb<u8>,Vec<u8>>, u32, u32) = image_file.load_image().expect("Failure loading image!");

    let mut image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image_data.0;
    let width: u32 = image_data.1;
    let height: u32 = image_data.2;

    let mut i: u32 = 1;
    while i < width-1 {
        let mut k: u32 = 1;
        while k < height-1 {
            // reduce each pixel's RGB values in the 2x2 grid to a single value
            let px_top_right: f32 = compute_rgb_distance(image_buf.get_pixel(i-1, k));
            let px_top_left: f32 = compute_rgb_distance(image_buf.get_pixel(i-1, k-1));
            let px_bottom_right: f32 = compute_rgb_distance(image_buf.get_pixel(i, k));
            let px_bottom_left: f32 = compute_rgb_distance(image_buf.get_pixel(i, k-1));
            // get the average of all four pixels
            let mean: f32 = (
                px_top_right + 
                px_top_left + 
                px_bottom_left + 
                px_bottom_right) / 4.0;
            // compute the distance between each pixel in the grid and the average of all four pixels
            let std_dev: f32 = compute_std_dev(
                mean, 
                px_top_right,
                px_top_left,
                px_bottom_left,
                px_bottom_right,
                width,
                height);
            // compare the resultant distance to the threshold const
            if std_dev > threshold {
                image_buf.put_pixel(i-1, k, GREEN_HIGHLIGHT_PX );
                image_buf.put_pixel(i-1, k-1, GREEN_HIGHLIGHT_PX );
                image_buf.put_pixel(i, k, GREEN_HIGHLIGHT_PX );
                image_buf.put_pixel(i, k-1, GREEN_HIGHLIGHT_PX );

            }
            else if blackout {
                image_buf.put_pixel(i-1, k, BLACKOUT_PX);
                image_buf.put_pixel(i-1, k-1, BLACKOUT_PX);
                image_buf.put_pixel(i, k, BLACKOUT_PX);
                image_buf.put_pixel(i, k-1, BLACKOUT_PX);
            }

            k += 2;
        }
        i += 2;
    }
    let _ = image_file.save_image(image_buf, width, height, &"edges");
}

fn compute_rgb_distance(pixel: &image::Rgb<u8>) -> f32 {
    f32::sqrt(
        f32::powf(pixel[0] as f32, 2.0) + 
        f32::powf(pixel[1] as f32, 2.0) + 
        f32::powf(pixel[2] as f32, 2.0)
    )
}

fn compute_std_dev(mean: f32, px_top_right: f32, px_top_left: f32, px_bottom_left: f32, px_bottom_right: f32, width: u32, height: u32) -> f32 {
    f32::sqrt(
        f32::powf(px_top_right-mean, 2.0) + 
        f32::powf(px_top_left-mean, 2.0) + 
        f32::powf(px_bottom_left-mean, 2.0) + 
        f32::powf(px_bottom_right-mean, 2.0) / (width as f32 * height as f32)
    )
}