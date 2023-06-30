use path_tracer::Render;

fn main() -> Result<(), image::ImageError> {
    let image = Render::new(2560, 1440).to_image();
    image.save_with_format("output.png", image::ImageFormat::Png)
}
