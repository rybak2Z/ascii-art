use image::DynamicImage;

pub fn resize_image(
    img: DynamicImage,
    scale: Option<f64>,
    new_width: Option<u32>,
    new_height: Option<u32>,
) -> DynamicImage {
    let mut width: u32 = img.width();
    let mut height: u32 = img.height();

    (width, height) = evaluate_dimensions(width, height, new_width, new_height, scale);

    let image_needs_resizing = width != img.width() || height != img.height();
    if !image_needs_resizing {
        return img;
    }

    img.resize_exact(width, height, image::imageops::FilterType::Triangle)
}

fn evaluate_dimensions(
    width: u32,
    height: u32,
    new_width: Option<u32>,
    new_height: Option<u32>,
    scale: Option<f64>,
) -> (u32, u32) {
    let final_width: u32;
    let final_height: u32;

    if let Some(scale) = scale {
        final_width = (width as f64 * scale).round() as u32;
        final_height = (height as f64 * scale).round() as u32;
    } else {
        let aspect_ratio = width as f64 / height as f64;
        (final_width, final_height) = match (new_width, new_height) {
            (Some(w), Some(h)) => (w, h),
            (Some(w), None) => (w, (w as f64 / aspect_ratio).round() as u32),
            (None, Some(h)) => ((h as f64 * aspect_ratio).round() as u32, h),
            (None, None) => (width, height),
        };
    }

    (final_width, final_height)
}
