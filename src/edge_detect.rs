use crate::rw_image;

fn edge_detect() -> String{
    // main edge detection function
    let image_data = rw_image::load_image(&file_path)
        .expect("Failed loading image!");

    "DETECTING EDGE"
}