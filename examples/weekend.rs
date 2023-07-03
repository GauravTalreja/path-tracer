use path_tracer::prelude::*;

fn main() -> Result<(), image::ImageError> {
    let scene = Scene {
        hittables: vec![
            Box::new(Sphere::new(
                0.5,
                DVec3 {
                    x: 0.,
                    y: 0.,
                    z: -1.,
                },
            )),
            Box::new(Sphere::new(
                100.,
                DVec3 {
                    x: 0.,
                    y: 100.5,
                    z: -1.,
                },
            )),
        ],
    };
    let image = Render::new(2560, 1440, 100, scene).to_image();
    image.save_with_format("output.png", image::ImageFormat::Png)
}
