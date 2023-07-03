use path_tracer::prelude::*;

fn main() -> Result<(), image::ImageError> {
    let ground = Arc::new(material::Lambertian {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.,
        },
    });

    let center = Arc::new(material::Lambertian {
        albedo: Color {
            x: 0.7,
            y: 0.3,
            z: 0.3,
        },
    });

    let left = Arc::new(material::Metal {
        albedo: Color {
            x: 0.8,
            y: 0.8,
            z: 0.8,
        },
        fuzz: 0.3,
    });

    let right = Arc::new(material::Metal {
        albedo: Color {
            x: 0.8,
            y: 0.6,
            z: 0.2,
        },
        fuzz: 1.,
    });

    let scene = Scene {
        hittables: vec![
            Box::new(hittable::Sphere::new(
                0.5,
                DVec3 {
                    x: 0.,
                    y: 0.,
                    z: -1.,
                },
                center,
            )),
            Box::new(hittable::Sphere::new(
                0.5,
                DVec3 {
                    x: -1.,
                    y: 0.,
                    z: -1.,
                },
                left,
            )),
            Box::new(hittable::Sphere::new(
                0.5,
                DVec3 {
                    x: 1.,
                    y: 0.,
                    z: -1.,
                },
                right,
            )),
            Box::new(hittable::Sphere::new(
                100.,
                DVec3 {
                    x: 0.,
                    y: 100.5,
                    z: -1.,
                },
                ground,
            )),
        ],
    };
    let image = Render::new(2560, 1440, 100, scene).to_image();
    image.save_with_format("output.png", image::ImageFormat::Png)
}
