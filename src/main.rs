#![allow(dead_code, unused_variables)]
use clap::Parser;

mod rw_image;
mod image_resize;
mod kmeans;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct InputArguments {
    command: Option<String>,
    path: Option<String>,
}

enum ImageCommand {
    Kmeans,
    ImageResize,
    EdgeDetect,
    Quit,
}

impl ImageCommand {
    fn route_command(self, file_path: String) -> String{
        match self {
            ImageCommand::EdgeDetect => {"EDGE DETECTING".to_string()},
            ImageCommand::ImageResize => {image_resize::image_resize(file_path)},
            ImageCommand::Kmeans => {kmeans::k_means(file_path)},
            ImageCommand::Quit => {"QUITTING".to_string()},
        }
    }
}

fn main() {
    let args: InputArguments = InputArguments::parse();

    let command_to_run: ImageCommand = match args.command {
        None => ImageCommand::Quit,
        Some(x) => {
            if x == "kmeans" {ImageCommand::Kmeans}
            else if x == "image_resize" {ImageCommand::ImageResize}
            else if x == "edge_detect" {ImageCommand::EdgeDetect}
            else {ImageCommand::Quit}
        }
    };
    // run the command the user wants to execute
    command_to_run.route_command(args
        .path
        .expect("No path given!"));
}