#![allow(dead_code, unused_variables)]
use clap::{Parser, Subcommand};

mod rw_image;
mod image_resize;
mod batch_resize;
mod kmeans;
mod edge_detect;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct InputArguments {
    #[command(subcommand)]
    command: ImageCommand,

    path: Option<String>,

    #[arg(long)]
    #[arg(default_value_t = 30.0)]
    #[arg(help="threshold for edge detection.")]
    threshold: f32,

    #[arg(long)]
    #[arg(default_value_t = false)]
    #[arg(help="blackout non-edge pixels in edge detection. Default is False.")]
    blackout: bool,

    #[arg(long)]
    #[arg(help="File format to filter by for batch resizing")]
    extension: Option<String>,
}

#[derive(Subcommand)]
enum ImageCommand {
    Kmeans,
    ImageResize,
    EdgeDetect,
    BatchResize,
    Quit,
}

fn route_command(arguments: InputArguments) {
    let image_details: rw_image::ImageFileDetails = rw_image::ImageFileDetails::get_filename_and_format(
        arguments.path.as_ref().expect("No path!"));

    match arguments.command {
        ImageCommand::EdgeDetect => {
            edge_detect::edge_detect(image_details, arguments.threshold, arguments.blackout);
        },
        ImageCommand::ImageResize => {
            image_resize::image_resize(image_details);
        },
        ImageCommand::Kmeans => {
            kmeans::k_means_fast(image_details);
        },
        ImageCommand::BatchResize => {
            batch_resize::batch_resize(arguments.path.expect("No path!"),
                arguments.extension.expect("No file format provided!"));
        }
        ImageCommand::Quit => {
            println!("QUITTING");
        },
    }
}

fn main() {
    let args: InputArguments = InputArguments::parse();
    route_command(args);
}