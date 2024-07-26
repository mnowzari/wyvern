#![allow(dead_code, unused_variables)]
use clap::Parser;

mod rw_image;
mod image_resize;
mod kmeans;
mod edge_detect;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct InputArguments {
    command: Option<String>,
    path: Option<String>,
    #[arg(short, long)]
    #[arg(help="threshold for edge detection only")]
    threshold: Option<f32>,
    #[arg(short, long)]
    #[arg(default_value_t = false)]
    #[arg(help="blackout non-edge pixels in edge detect")]
    blackout: bool,
}

enum ImageCommand {
    Kmeans,
    ImageResize,
    EdgeDetect,
    Quit,
}

impl ImageCommand {
    fn route_command(self, arguments: InputArguments) -> String{
        match self {
            ImageCommand::EdgeDetect => {
                edge_detect::edge_detect(arguments
                    .path
                    .expect("No path!"), 
                arguments.threshold.expect("No threshold!"), 
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
            ImageCommand::Quit => {"QUITTING".to_string()},
        }
    }
}

fn main() {
    let args: InputArguments = InputArguments::parse();

    let command_to_run: ImageCommand = match &args.command {
        None => ImageCommand::Quit,
        Some(x) => {
            if x == "kmeans" {ImageCommand::Kmeans}
            else if x == "image_resize" {ImageCommand::ImageResize}
            else if x == "edge_detect" {ImageCommand::EdgeDetect}
            else {ImageCommand::Quit}
        }
    };
    // run the command the user wants to execute
    command_to_run.route_command(args);
}