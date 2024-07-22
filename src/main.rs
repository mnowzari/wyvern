#![allow(dead_code, unused_variables)]
use std::env;

mod image_loader;
mod image_resize;
mod kmeans;

struct InputArguments {
    command: String,
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
            ImageCommand::ImageResize => {"IMAGE RESIZING".to_string()},
            ImageCommand::Kmeans => {kmeans::k_means(file_path)},
            ImageCommand::Quit => {"QUITTING".to_string()},
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_and_path: InputArguments = parse_args(args);

    let command_to_run: ImageCommand = match cmd_and_path.command {
        x if x=="kmeans" => ImageCommand::Kmeans,
        x if x=="image_resize" => ImageCommand::ImageResize,
        x if x=="edge_detect" => ImageCommand::EdgeDetect,
        _ => ImageCommand::Quit,
    };
    // run the command the user wants to execute
    println!("{}", command_to_run.route_command(cmd_and_path.path));
}

fn parse_args(arguments: Vec<String>) -> InputArguments {
    let mut args_struct: InputArguments = InputArguments {
        command: "".to_string(),
        path: "".to_string(),
    };

    if arguments.len() == 3 {
        args_struct.command = arguments[1].to_string();
        args_struct.path = arguments[2].to_string();
        args_struct
    }
    else {
        args_struct
    }
}