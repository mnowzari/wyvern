use crate::image_loader;

const DOWNSCALE_FACTOR: u32 = 2;

pub fn image_resize(file_path: String) -> String {
    // main resizing function
    let image_data = image_loader::load_image(&file_path)
        .expect("Failed loading image!");

    let image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image_data.0;
    let width: u32 = image_data.1;
    let height: u32 = image_data.2;
    let mut output_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(
        width/DOWNSCALE_FACTOR, height/DOWNSCALE_FACTOR);

    let mask: i32 = 0x3f3f3f;
    let mut i: u32 = 1;
    while i < width-1 {
        let mut k: u32 = 1;
        while k < height-1 {
            let px_top_right: &image::Rgb<u8> = image_buf.get_pixel(i-1, k);
            let px_top_left: &image::Rgb<u8> = image_buf.get_pixel(i-1, k-1);
            let px_bottom_right: &image::Rgb<u8> = image_buf.get_pixel(i, k);
            let px_bottom_left: &image::Rgb<u8> = image_buf.get_pixel(i, k-1);

            let px_avg: image::Rgb<u8> = average_pixel_values(
                px_top_right,
                px_top_left,
                px_bottom_right,
                px_bottom_left);

            output_buf.put_pixel(
                i/DOWNSCALE_FACTOR,
                k/DOWNSCALE_FACTOR,
                px_avg);
            k += 2;
        }
        i += 2;
    }

    let _ = image_loader::save_image(output_buf, width, height, &file_path, &"minimized".to_string());
    file_path
}

fn average_pixel_values(top_right_pixel: &image::Rgb<u8>, top_left_pixel: &image::Rgb<u8>, 
    bottom_right_pixel: &image::Rgb<u8>, bottom_left_pixel: &image::Rgb<u8>) -> image::Rgb<u8> {

    let red_avg: u32 = (top_right_pixel[0] as u32 + top_left_pixel[0] as u32 + bottom_right_pixel[0] as u32 + bottom_left_pixel[0] as u32) / 4;
    let gre_avg: u32 = (top_right_pixel[1] as u32 + top_left_pixel[1] as u32 + bottom_right_pixel[1] as u32 + bottom_left_pixel[1] as u32) / 4;
    let blu_avg: u32 = (top_right_pixel[2] as u32 + top_left_pixel[2] as u32 + bottom_right_pixel[2] as u32 + bottom_left_pixel[2] as u32) / 4;
    image::Rgb([(red_avg % 255) as u8, (gre_avg % 255) as u8, (blu_avg % 255) as u8])
}