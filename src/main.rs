#[allow(dead_code, unused)]
use clap::{Parser, Subcommand};

mod batch_resize;
mod edge_detect;
mod image_resize;
mod kmeans;
mod rw_image;
mod threadpool;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    propagate_version = true
)]
struct InputArguments {
    #[command(subcommand)]
    command: ImageCommand,
}

#[derive(Subcommand)]
enum ImageCommand {
    ImageResize {
        #[arg(required = true)]
        path: Option<String>,
    },

    Kmeans {
        #[arg(required = true)]
        path: Option<String>,
    },

    EdgeDetect {
        #[arg(required = true)]
        path: Option<String>,

        #[arg(long, default_value_t = 30.0, help = "Threshold for edge detection.")]
        threshold: f32,

        #[arg(
            long,
            default_value_t = false,
            help = "Blackout non-edge pixels in edge detection."
        )]
        blackout: bool,
    },

    BatchResize {
        #[arg(required = true)]
        path: Option<String>,

        #[arg(required = true, help = "File format to filter by for batch resizing.")]
        extension: Option<String>,
    },
}

fn route_command(args: InputArguments) {
    match args.command {
        ImageCommand::EdgeDetect {
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
        ImageCommand::ImageResize { path } => {
            let _ = image_resize::image_resize(&mut rw_image::ImageDetails::new_image(
                path.as_ref().expect("No path!"),
            ));
        }
        ImageCommand::Kmeans { path } => {
            let _ = kmeans::k_means_fast(rw_image::ImageDetails::new_image(
                path.as_ref().expect("No path!"),
            ));
        }
        ImageCommand::BatchResize { path, extension } => {
            let _ = batch_resize::batch_resize(
                path.expect("No path!"),
                extension.expect("No file format provided!"),
            );
        }
    }
}

fn main() {
    let args: InputArguments = InputArguments::parse();
    route_command(args);
}
