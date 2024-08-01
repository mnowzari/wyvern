use clap::{Parser, Subcommand};

mod rw_image;
mod image_resize;
mod batch_resize;
mod kmeans;
mod edge_detect;

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

        #[arg(
            long,
            default_value_t = 30.0,
            help="Threshold for edge detection."
        )]
        threshold: f32,

        #[arg(
            long,
            default_value_t = false,
            help="Blackout non-edge pixels in edge detection."
        )]
        blackout: bool,
    },

    BatchResize {
        #[arg(required = true)]
        path: Option<String>,

        #[arg(
            required = true,
            help="File format to filter by for batch resizing."
        )]
        extension: Option<String>,
    },
}

fn route_command(args: InputArguments) {

    match args.command {
        ImageCommand::EdgeDetect {path, threshold, blackout} => {
            edge_detect::edge_detect(
                rw_image::ImageDetails::get_filename_and_format(path
                    .as_ref()
                    .expect("No path!")),
                threshold,
                blackout);
        },
        ImageCommand::ImageResize {path} => {
            let _ = image_resize::image_resize(
                rw_image::ImageDetails::get_filename_and_format(path
                    .as_ref()
                    .expect("No path!"))
            );
        },
        ImageCommand::Kmeans {path} => {
            kmeans::k_means_fast(
                rw_image::ImageDetails::get_filename_and_format(path
                    .as_ref()
                    .expect("No path!"))
            );
        },
        ImageCommand::BatchResize {path, extension} => {
            batch_resize::batch_resize(
                path.expect("No path!"), 
                extension.expect("No file format provided!"));
        }
    }
}

fn main() {
    let args: InputArguments = InputArguments::parse();
    route_command(args);
}