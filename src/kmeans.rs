use crate::image_loader;

pub fn k_means(file_path: String) -> String {
   let _ = image_loader::load_image(&file_path);
   
   "None".to_string()
}