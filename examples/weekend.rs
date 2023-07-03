use path_tracer::*;

fn main() -> Result<(), image::ImageError> {
    let ground = Arc::new(material::Lambertian {
        albedo: hex_color(0xb4befe),
    });

    let center = Arc::new(material::Lambertian {
        albedo: hex_color(0xf5c2e7),
    });

    let left = Arc::new(material::Dielectric {
        refractive_index: 1.5,
    });

    let right = Arc::new(material::Metal {
        albedo: hex_color(0xcba6f7),
        fuzz: 0.,
    });

    let scene = Scene {
        hittables: vec![
            Box::new(hittable::Sphere::new(0.5, DVec3::new(0., 0., -1.), center)),
            Box::new(hittable::Sphere::new(
                0.5,
                DVec3::new(-1., 0., -1.),
                left.clone(),
            )),
            Box::new(hittable::Sphere::new(-0.4, DVec3::new(-1., 0., -1.), left)),
            Box::new(hittable::Sphere::new(0.5, DVec3::new(1., 0., -1.), right)),
            Box::new(hittable::Sphere::new(
                100.,
                DVec3::new(0., 100.5, -1.),
                ground,
            )),
        ],
    };
    let image = Render::new(2560, 1440, 100, scene).to_image();
    image.save_with_format("output.png", image::ImageFormat::Png)
}
