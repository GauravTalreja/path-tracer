use path_tracer::*;

const WIDTH: u32 = 720;
const HEIGHT: u32 = 720;
const TIME_MIN: f64 = 0.;
const TIME_MAX: f64 = 1.;

fn main() -> Result<(), image::ImageError> {
    let red = Arc::new(material::Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(material::Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(material::Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(material::DiffuseLight::new(Color::splat(15.)));
    let hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
        Arc::new(hittable::Rectangle::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            red,
            hittable::Plane::YZ,
        )),
        Arc::new(hittable::Rectangle::new(
            0.,
            555.,
            0.,
            555.,
            0.,
            green,
            hittable::Plane::YZ,
        )),
        Arc::new(hittable::Rectangle::new(
            213.,
            343.,
            227.,
            332.,
            554.,
            light,
            hittable::Plane::ZX,
        )),
        Arc::new(hittable::Rectangle::new(
            0.,
            555.,
            0.,
            555.,
            0.,
            white.clone(),
            hittable::Plane::ZX,
        )),
        Arc::new(hittable::Rectangle::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
            hittable::Plane::ZX,
        )),
        Arc::new(hittable::Rectangle::new(
            0.,
            555.,
            0.,
            555.,
            555.,
            white.clone(),
            hittable::Plane::XY,
        )),
    ];
    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX, Color::ZERO);

    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        DVec3::new(278., 278., -800.),
        DVec3::new(278., 278., 0.),
        DVec3::new(0., -1., 0.),
        40.,
        aspect_ratio,
        0.,
        10.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 1000, scene, camera).to_image();
    image.save("examples/cornell_box.png")
}
