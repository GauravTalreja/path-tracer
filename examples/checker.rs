use path_tracer::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f64 = 0.001;
const TIME_MAX: f64 = f64::MAX;

fn main() -> Result<(), image::ImageError> {
    let purple = Arc::new(texture::SolidColor::new(hex_color(0x1e1e2e)));
    let pink = Arc::new(texture::SolidColor::new(hex_color(0xf5c2e7)));
    let checker = Arc::new(material::Lambertian {
        albedo: Arc::new(texture::Checker {
            odd: purple,
            even: pink,
        }),
    });
    let hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
        Arc::new(hittable::Sphere::new(
            10.,
            DVec3::new(0., -10., -1.),
            checker.clone(),
        )),
        Arc::new(hittable::Sphere::new(
            10.,
            DVec3::new(0., 10., -1.),
            checker,
        )),
    ];
    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX);

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        DVec3::new(13., -2., 3.),
        DVec3::new(0., 0., 0.),
        DVec3::new(0., -1., 0.),
        40.,
        aspect_ratio,
        0.,
        10.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 100, scene, camera).to_image();
    image.save_with_format("examples/checker.png", image::ImageFormat::Png)
}
