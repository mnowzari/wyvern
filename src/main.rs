#![allow(dead_code, unused_variables)]
use clap::Parser;

mod rw_image;
mod image_resize;
mod kmeans;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct InputArguments {
    #[arg(short, long)]
    command: String,
    #[arg(short, long)]
    path: String,
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
        x if x=="kmeans" => ImageCommand::Kmeans,
        x if x=="image_resize" => ImageCommand::ImageResize,
        x if x=="edge_detect" => ImageCommand::EdgeDetect,
        _ => ImageCommand::Quit,
    };
    // run the command the user wants to execute
    command_to_run.route_command(args.path);
}