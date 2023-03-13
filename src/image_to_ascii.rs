use crate::Method;
use image::{DynamicImage, GenericImageView};

pub fn print_art(
    img: &DynamicImage,
    method: Method,
    invert: bool,
    chars_per_pixel: u32,
    character_ramp: &String,
) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let (r, g, b) = get_pixel_rgb_as_f64(img, x, y);
            let brightness = calculate_brightness(r, g, b, method);
            let character = get_char_from_brightness(brightness, invert, character_ramp);
            print!("{}", construct_ascii_string(character, chars_per_pixel));
        }
        println!();
    }
}

fn get_pixel_rgb_as_f64(img: &DynamicImage, x: u32, y: u32) -> (f64, f64, f64) {
    let pixel = img.get_pixel(x, y);
    (pixel.0[0] as f64, pixel.0[1] as f64, pixel.0[2] as f64)
}

fn calculate_brightness(r: f64, g: f64, b: f64, method: Method) -> f64 {
    match method {
        Method::Average => (r + g + b) / 3.0,
        Method::Lightness => {
            let min = [r, g, b].into_iter().reduce(f64::min).unwrap();
            let max = [r, g, b].into_iter().reduce(f64::max).unwrap();
            (min + max) / 2.0
        }
        Method::Luminosity => 0.21 * r + 0.72 * g + 0.07 * b,
    }
}

fn get_char_from_brightness(brightness: f64, invert: bool, ramp: &String) -> char {
    let relative_brightness = brightness / 255.0;
    let index = relative_brightness * (ramp.len() - 1) as f64;
    let mut index = index.round() as usize;
    if invert {
        index = (ramp.len() - 1) - index;
    }

    ramp.chars().nth(index).unwrap()
}

fn construct_ascii_string(character: char, chars_per_pixel: u32) -> String {
    let mut string = String::with_capacity(chars_per_pixel.try_into().unwrap());
    for _ in 0..chars_per_pixel {
        string.push(character);
    }
    string
}
