use path_tracer::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f32 = 0.;
const TIME_MAX: f32 = 1.;

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
            Vec3A::new(0., 0., -2.1),
            earthmap,
        )),
        Arc::new(hittable::Sphere::new(
            2.,
            Vec3A::new(0., 0., 2.1),
            bad_apple,
        )),
    ];
    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX, Color::new(0.70, 0.80, 1.00));

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(
        Vec3A::new(13., -2., 3.),
        Vec3A::new(0., 0., 0.),
        Vec3A::new(0., 1., 0.),
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
