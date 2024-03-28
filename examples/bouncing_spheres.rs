use path_tracer::*;
const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;
const TIME_MIN: f32 = 0.;
const TIME_MAX: f32 = 1.;

fn main() -> Result<(), image::ImageError> {
    let rng = RandomNumberGenerator::new();
    let scene = Scene::random_moving_spheres(11, rng, TIME_MIN, TIME_MAX);

    let look_from = Vec3A::new(13., 2., 3.);
    let look_at = Vec3A::new(0., 0., 0.);
    let vup = Vec3A::new(0., 1., 0.);
    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
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

    let image = Render::new(WIDTH, HEIGHT, 500, scene, camera).to_image();
    image.save_with_format("examples/bouncing_spheres.png", image::ImageFormat::Png)
}
