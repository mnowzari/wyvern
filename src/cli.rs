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

        #[arg(
            long,
            default_value_t = 30.0,
            help = "Threshold for edge detection.
        The higher the number, the more edges will be detected, and might give a noisier output."
        )]
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
            help = "Threshold for pixel sort distance.
            The higher the number, the more likely it is that pixels will be affected by the sorting mechanism."
        )]
        threshold: f32,

        #[arg(
            long,
            help = "General direction of the sorted pixels. Accepts the following strings: vertical, horizontal, diagonal.
            All other input will default to diagonal."
        )]
        direction: Option<String>,
    },

    Denoise {
        #[arg(required = true)]
        path: Option<String>,

        #[arg(
            long,
            default_value_t = 2.0,
            help = "Floating point value (1.0 - 100.0) for how aggressive denoising should be.
            The higher the number, the less aggressive the denoising."
        )]
        threshold: f32,

        #[arg(
            long,
            default_value_t = false,
            help = "Highlight detected noise in the ouput file."
        )]
        highlight: bool,
    },

    Greyscale {
        #[arg(required = true)]
        path: Option<String>,
    },
}

#[derive(Clone)]
pub enum PixelSortDir {
    Vertical,
    Horizontal,
    Diagonal,
}

pub fn denoise_threshold_between_bounds(f: &f32) -> Result<f32, ()> {
    let f_val: f32 = *f;
    if f_val >= 1.0 && f_val <= 100.0 {
        Ok(f_val)
    } else {
        Err(())
    }
}
