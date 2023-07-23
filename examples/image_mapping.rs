use path_tracer::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f64 = 0.;
const TIME_MAX: f64 = 1.;

fn main() -> Result<(), image::ImageError> {
    let earthmap = Arc::new(material::Lambertian {
        albedo: Arc::new(texture::Image::new("textures/earthmap.jpg")),
    });
    let bad_apple = Arc::new(material::Lambertian {
        albedo: Arc::new(texture::Image::new("textures/bad_apple.png")),
    });
    let hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
        Arc::new(hittable::Sphere::new(
            2.,
            DVec3::new(0., 0., -2.1),
            earthmap,
        )),
        Arc::new(hittable::Sphere::new(
            2.,
            DVec3::new(0., 0., 2.1),
            bad_apple,
        )),
    ];
    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX);

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        DVec3::new(13., -2., 3.),
        DVec3::new(0., 0., 0.),
        DVec3::new(0., -1., 0.),
        20.,
        aspect_ratio,
        0.,
        10.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 100, scene, camera).to_image();
    image.save("examples/image_mapping.png")
}
