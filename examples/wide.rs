use path_tracer::*;
use std::f64::consts::PI;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f64 = 0.001;
const TIME_MAX: f64 = f64::MAX;

fn main() -> Result<(), image::ImageError> {
    let left = Arc::new(material::Lambertian::new(hex_color(0xb4befe)));
    let right = Arc::new(material::Lambertian::new(hex_color(0xf5c2e7)));

    let r = (PI / 4.).cos();

    let hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
        Arc::new(hittable::Sphere::new(r, DVec3::new(-r, 0., -1.), left)),
        Arc::new(hittable::Sphere::new(r, DVec3::new(r, 0., -1.), right)),
    ];
    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX, Color::new(0.70, 0.80, 1.00));

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        DVec3::ZERO,
        DVec3::new(0., 0., -1.),
        DVec3::new(0., 1., 0.),
        90.,
        aspect_ratio,
        0.,
        1.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 100, scene, camera).to_image();
    image.save_with_format("examples/wide.png", image::ImageFormat::Png)
}
