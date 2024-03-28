use path_tracer::*;

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f32 = 0.001;
const TIME_MAX: f32 = f32::MAX;

fn main() -> Result<(), image::ImageError> {
    let purple = Arc::new(texture::SolidColor::new(hex_color(0x1e1e2e)));
    let pink = Arc::new(texture::SolidColor::new(hex_color(0xf5c2e7)));
    let checker = Arc::new(material::Lambertian {
        albedo: Arc::new(texture::Checker {
            odd: purple,
            even: pink,
        }),
    });
    let diffuse_light = Arc::new(material::DiffuseLight::new(Color::splat(4.)));
    let hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
        Arc::new(hittable::Sphere::new(
            1000.,
            Vec3A::new(0., -1000., 0.),
            checker.clone(),
        )),
        Arc::new(hittable::Sphere::new(2., Vec3A::new(0., 2., 0.), checker)),
        Arc::new(hittable::Sphere::new(
            1.5,
            Vec3A::new(0., 6.5, 0.),
            diffuse_light,
        )),
    ];
    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX, Color::ZERO);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(
        Vec3A::new(23., 3., 6.),
        Vec3A::new(0., 2., 0.),
        Vec3A::new(0., 1., 0.),
        20.,
        aspect_ratio,
        0.,
        10.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 400, scene, camera).to_image();
    image.save_with_format("examples/simple_light.png", image::ImageFormat::Png)
}
