#[allow(dead_code, unused)]
use clap::Parser;

mod batch_downscale;
mod cli;
mod denoise;
mod edge_detect;
mod greyscale;
mod image_downscale;
mod kmeans;
mod pixelsort;
mod rw_image;
mod threadpool;
mod utils;

fn route_command(args: cli::InputArguments) {
    match args.command {
        cli::ImageCommand::EdgeDetect {
            path,
            threshold,
            blackout,
        } => {
            let _ = edge_detect::edge_detect(
                &mut rw_image::ImageDetails::new_image(path.as_ref().expect("No path!")),
                threshold,
                blackout,
            );
        }
        cli::ImageCommand::ImageDownscale { path } => {
            let _ = image_downscale::image_downscale(&mut rw_image::ImageDetails::new_image(
                path.as_ref().expect("No path!"),
            ));
        }
        cli::ImageCommand::Kmeans { path } => {
            let _ = kmeans::k_means_fast(&mut rw_image::ImageDetails::new_image(
                path.as_ref().expect("No path!"),
            ));
        }
        cli::ImageCommand::BatchDownscale { path, extension } => {
            let _ = batch_downscale::batch_downscale(
                path.expect("No path!"),
                extension.expect("No file format provided!"),
            );
        }
        cli::ImageCommand::PixelSort {
            path,
            threshold,
            direction,
        } => {
            let _ = pixelsort::pixel_sort(
                &mut rw_image::ImageDetails::new_image(path.as_ref().expect("No path!")),
                threshold,
                match direction {
                    Some(x) => {
                        if x == "horizontal" {
                            cli::PixelSortDir::Horizontal
                        } else if x == "vertical" {
                            cli::PixelSortDir::Vertical
                        } else {
                            cli::PixelSortDir::Diagonal
                        }
                    }
                    None => cli::PixelSortDir::Diagonal,
                },
            );
        }
        cli::ImageCommand::Denoise {
            path,
            threshold,
            highlight,
        } => {
            let _ = denoise::denoise(
                &mut rw_image::ImageDetails::new_image(path.as_ref().expect("No path!")),
                match cli::denoise_threshold_between_bounds(&threshold) {
                    Ok(x) => x,
                    Err(_) => {
                        panic!("Please give a number between 1.0 and 100.0!");
                    }
                },
                highlight,
            );
        }
        cli::ImageCommand::Greyscale { path } => {
            let _ = greyscale::greyscale_convert(rw_image::ImageDetails::new_image(
                path.as_ref().expect("No path!"),
            ));
        }
    }
}

fn main() {
    let args: cli::InputArguments = cli::InputArguments::parse();
    route_command(args);
}
