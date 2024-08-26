use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    propagate_version = true
)]
pub struct InputArguments {
    #[command(subcommand)]
    pub command: ImageCommand,
}

#[derive(Subcommand)]
pub enum ImageCommand {
    ImageDownscale {
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

    BatchDownscale {
        #[arg(required = true)]
        path: Option<String>,

        #[arg(required = true, help = "File format to filter by for batch resizing.")]
        extension: Option<String>,
    },

    PixelSort {
        #[arg(required = true)]
        path: Option<String>,

        #[arg(
            long,
            default_value_t = 70.0,
            help = "Threshold for pixel sort distance."
        )]
        threshold: f32,

        #[arg(
            long,
            help = "General direction of the sorted pixels. Accepts the following strings: vertical, horizontal, diagonal.
            All other input will default to diagonal."
        )]
        direction: Option<String>,
    },
}

#[derive(Clone)]
pub enum PixelSortDir {
    Vertical,
    Horizontal,
    Diagonal,
}
