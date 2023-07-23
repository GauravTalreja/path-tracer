use path_tracer::*;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f64 = 0.;
const TIME_MAX: f64 = 1.;

fn main() -> Result<(), image::ImageError> {
    let rng = RandomNumberGenerator::new(TIME_MIN, TIME_MAX);
    let scene = Scene::random_moving_spheres(11, rng, TIME_MIN, TIME_MAX);

    let look_from = DVec3::new(13., -2., 3.);
    let look_at = DVec3::new(0., 0., 0.);
    let vup = DVec3::new(0., 1., 0.);
    let aspect_ratio = WIDTH as f64 / HEIGHT as f64;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        aspect_ratio,
        0.1,
        10.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 100, scene, camera).to_image();
    image.save_with_format("examples/bouncing_spheres.png", image::ImageFormat::Png)
}
