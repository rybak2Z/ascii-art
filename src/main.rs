use ascii_art::{print_art, resize_image, Cli};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let mut img = image::open(cli.file_path).expect("Could not read image file.");

    img = resize_image(img, cli.scale, cli.width, cli.height);

    print_art(
        &img,
        cli.method,
        cli.invert,
        cli.chars_per_pixel,
        &cli.character_ramp,
    );
}
