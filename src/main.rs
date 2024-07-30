#![allow(dead_code, unused_variables)]
use clap::{Parser, Subcommand};

mod rw_image;
mod image_resize;
mod kmeans;
mod edge_detect;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct InputArguments {
    #[command(subcommand)]
    command: ImageCommand,

    path: Option<String>,

    #[arg(short, long)]
    #[arg(default_value_t = 30.0)]
    #[arg(help="threshold for edge detection.")]
    threshold: f32,

    #[arg(short, long)]
    #[arg(default_value_t = false)]
    #[arg(help="blackout non-edge pixels in edge detection. Default is False.")]
    blackout: bool,
}

// #[derive(Parser)]
// #[command(version, about, long_about = None)]
// #[command(propagate_version = true)]
// struct EdgeDetectOptionals {
//     #[arg(short, long)]
//     #[arg(default_value_t = 30.0)]
//     #[arg(help="threshold for edge detection.")]
//     threshold: f32,

//     #[arg(short, long)]
//     #[arg(default_value_t = false)]
//     #[arg(help="blackout non-edge pixels in edge detection. Default is False.")]
//     blackout: bool,
// }

#[derive(Subcommand)]
enum ImageCommand {
    Kmeans,
    ImageResize,
    EdgeDetect,
    BatchResize,
    Quit,
}

fn route_command(arguments: InputArguments) -> String{
    match arguments.command {
        ImageCommand::EdgeDetect => {
            edge_detect::edge_detect(arguments
                .path
                .expect("No path!"), 
            arguments.threshold,
            arguments.blackout)
        },
        ImageCommand::ImageResize => {
            image_resize::image_resize(arguments
                .path
                .expect("No path!"))
        },
        ImageCommand::Kmeans => {
            kmeans::k_means_fast(arguments
                .path
                .expect("No path!"))
        },
        ImageCommand::BatchResize => {
            "BATCH RESIZING".to_string()
        }
        ImageCommand::Quit => {"QUITTING".to_string()},
    }
}

fn main() {
    let args: InputArguments = InputArguments::parse();
    route_command(args);
}