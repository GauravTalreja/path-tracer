use path_tracer::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;
const TIME_MIN: f32 = 0.;
const TIME_MAX: f32 = 1.;

fn main() -> Result<(), image::ImageError> {
    let red = Arc::new(material::Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(material::Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(material::Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(material::DiffuseLight::new(Color::splat(15.)));

    let box1_min = Vec3A::new(265., 0., 295.);
    let box1_max = Vec3A::new(430., 330., 460.);
    let box1_center = (box1_min + box1_max) * 0.5;
    let box1_rot = Quat::from_rotation_y(15.0_f32.to_radians());
    let box1= hittable::Box::new(box1_min, box1_max, white.clone());
    let box1 = Arc::new(hittable::Transform::new(Arc::new(box1), box1_rot, box1_center));

    let box2_min = Vec3A::new(130., 0., 65.);
    let box2_max = Vec3A::new(295., 165., 130.);
    let box2_center = (box2_min + box2_max) * 0.5;
    let box2_rot = Quat::from_rotation_y((-18.0_f32).to_radians());
    let box2= hittable::Box::new(box2_min, box2_max, white.clone());
    let box2 = Arc::new(hittable::Transform::new(Arc::new(box2), box2_rot, box2_center));


    let hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
        Arc::new(hittable::Quad::new(Vec3A::new(555.0, 0.0, 0.0), Vec3A::new(0.0, 555.0, 0.0), Vec3A::new(0.0, 0.0, 555.0), green.clone())),
        Arc::new(hittable::Quad::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(0.0, 555.0, 0.0), Vec3A::new(0.0, 0.0, 555.0), red.clone())),
        Arc::new(hittable::Quad::new(Vec3A::new(343.0, 554.0, 332.0), Vec3A::new(-130.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, -105.0), light.clone())),
        Arc::new(hittable::Quad::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(555.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, 555.0), white.clone())),
        Arc::new(hittable::Quad::new(Vec3A::new(555.0, 555.0, 555.0), Vec3A::new(-555.0, 0.0, 0.0), Vec3A::new(0.0, 0.0, -555.0), white.clone())),
        Arc::new(hittable::Quad::new(Vec3A::new(0.0, 0.0, 555.0), Vec3A::new(555.0, 0.0, 0.0), Vec3A::new(0.0, 555.0, 0.0), white.clone())),
        box1,
        box2
    ];


    let scene = Scene::new(&hittables, TIME_MIN, TIME_MAX, Color::ZERO);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let camera = Camera::new(
        Vec3A::new(278., 273., -800.),
        Vec3A::new(278., 278., 0.),
        Vec3A::new(0., 1., 0.),
        40.,
        aspect_ratio,
        0.,
        10.,
        TIME_MIN,
        TIME_MAX,
    );

    let image = Render::new(WIDTH, HEIGHT, 200, scene, camera).to_image();
    image.save("examples/cornell_box.png")
}
