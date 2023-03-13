use clap::{Parser, ValueEnum};
use std::fmt;

mod image_transformation;
pub use image_transformation::resize_image;

mod image_to_ascii;
pub use image_to_ascii::print_art;

const CHARACTER_RAMP: &str = " .,-:=+*/?#%@";

#[derive(Parser)]
#[command(about = "Transforms an image to ASCII art and prints it to the console.", long_about = None)]
#[command(group(
    clap::ArgGroup::new("dimensions")
        .args(["width", "height"])
        .multiple(true)
        .conflicts_with("scale")
))]
pub struct Cli {
    /// Path to an image file.
    pub file_path: String,

    /// The factor by which to scale the image before transforming it to ASCII art.
    #[arg(short, long)]
    pub scale: Option<f64>,

    /// Width to scale the image to. If no value is provided for height, this preserves the aspect ratio.
    #[arg(short = 'W', long)] // capital letter for consistency with height argument
    pub width: Option<u32>,

    /// Height to scale the image to. If no value is provided for width, this preserves the aspect ratio.
    #[arg(short = 'H', long)] // capital letter because 'h' is reserved for help
    pub height: Option<u32>,

    /// Method to be used to determine the brightness (and thus the corresponding ASCII character) of a pixel.
    #[arg(short, long, default_value_t = Method::Luminosity)]
    pub method: Method,

    /// Inverts the brightness.
    #[arg(short, long)]
    pub invert: bool,

    /// Number of chars to be printed for one pixel to account for non-square shape of characters vs square shape of pixels.
    #[arg(short, long, default_value_t = 2)]
    pub chars_per_pixel: u32,

    /// A string of characters to map the different brightness levels to. Sorted in ascending order (by "density").
    #[arg(short = 'r', long = "ramp", default_value_t = String::from(CHARACTER_RAMP))]
    pub character_ramp: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Method {
    /// Average of the red, green, and blue values.
    Average,
    /// Average of maximum and minimum values out of the red, green, and blue values.
    Lightness,
    /// Weighted average of the red, green, and blue values to account for human perception.
    Luminosity,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Average => write!(f, "average"),
            Method::Lightness => write!(f, "lightness"),
            Method::Luminosity => write!(f, "luminosity"),
        }
    }
}
