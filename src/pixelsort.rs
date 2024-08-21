use image::io::Reader;
use rand::Rng;
use std::env;
use std::error::Error;

use crate::rw_image;

pub fn pixel_sort(mut image_details: rw_image::ImageDetails) -> Result<(), Box<dyn Error>> {
    let mut image_buf: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image_details.load_image().expect("Failure loading image!");

    let width = image_details.width;
    let height = image_details.height;

    for row in 0..width {
        for col in 0..height {
            image_buf.get_pixel(row, col);
        }
    }
    Ok(())
}

// fn basic_pixel_sort(mut linear: Vec<Vec3<f32>>) -> Vec<Vec3<f32>> {
//     let random_pixel_idx = rand::thread_rng().gen_range(2..=linear.len()-1);
//     println!("{random_pixel_idx}");
//     for idx in 1..linear.len() {
//         let r = linear[idx][0];
//         let g = linear[idx][1];
//         let b = linear[idx][2];

//         let x = linear[random_pixel_idx][0];
//         let y = linear[random_pixel_idx][1];
//         let z = linear[random_pixel_idx][2];
//         // calculate distance
//         let distance = f32::sqrt( f32::powf(r-x, 2.0) + f32::powf(g-y, 2.0) + f32::powf(b-z, 2.0) );
//         // check distance and swap elements as needed
//         if distance < 70.0 {
//             linear[idx] = linear[idx-1]
//         }
//     }
//     return linear
// }

// fn main() -> Result<(), Box<dyn Error>> {
//     println!("<<<< RUST Image Manipulator >>>>");
//     // Get arguments
//     let args: Vec<String> = env::args().collect();

//     let file_path = &args[1];
//     println!("{file_path}");

//     let img = Reader::open(file_path)?.decode()?;

//     let mut rgb = img.into_rgb8();
//     let mut linear = vec![Vec3::<f32>::zero(); rgb.as_raw().len()];

//     // let width = rgb.width();
//     // let height = rgb.height();

//     // zip the pixels into the linear buffer we created above
//     rgb.pixels()
//         .zip(linear.iter_mut())
//         .for_each(|(rgb, linear)| {
//             let rgbvec = Vec3::<u8>::from(rgb.0);
//             // *linear = rgbvec.numcast::<f32>().unwrap().map(|x| x / 255.0);
//             *linear = rgbvec.numcast::<f32>().unwrap();
//         });

//     let modified = basic_pixel_sort(linear);
//     // zip the output back into the img rgb
//     modified
//         .into_iter()
//             .zip(rgb.pixels_mut())
//             .for_each(|(linear, rgb)| {
//                 // let transformed = (linear * 255.0).clamped(0.0, 255.0);
//                 rgb.0 = linear.numcast().unwrap().into_array();
//             });

//     println!("./{file_path}_modified.png");
//     rgb.save(format!("{file_path}_modified.png"))?;
//     Ok(())
// }
