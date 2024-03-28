use path_tracer::*;
use path_tracer::material::*;
use path_tracer::hittable::*;
use rand::{Rng, thread_rng, distributions::Uniform, distributions::Distribution};

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;
const SAMPLES: u64 = 100;
const TIME_MIN: f32 = 0.;
const TIME_MAX: f32 = 1.;

fn main() -> Result<(), image::ImageError> {
    let mut rng = rand::thread_rng();
    let mut world = Vec::<Arc<dyn hittable::Hittable>>::new();
    
    let ground_material = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));
    
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..101.0);
            let z1 = z0 + w;

            let box_min = Vec3A::new(x0 as f32, y0 as f32, z0 as f32);
            let box_max = Vec3A::new(x1 as f32, y1 as f32, z1 as f32);
            let b = Arc::new(Box::new(box_min, box_max, ground_material.clone()));
            world.push(b);
        }
    }

    // Light
    let light_material = Arc::new(DiffuseLight::new(Color::splat(7.0)));
    let light_quad = Arc::new(Quad::new(
        Vec3A::new(123.0, 554.0, 147.0),
        Vec3A::new(300.0, 0.0, 0.0),
        Vec3A::new(0.0, 0.0, 265.0),
        light_material,
    ));
    world.push(light_quad);

    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    let center_start = Vec3A::new(400.0, 400.0, 200.0);
    let center_end = center_start + Vec3A::new(30.0, 0.0, 0.0);
    let moving_sphere = Arc::new(Sphere::new_moving(50.0, center_start, moving_sphere_material, TIME_MAX, TIME_MIN, center_end));
    world.push(moving_sphere);

    let glass_sphere = Arc::new(Sphere::new(50.0, Vec3A::new(260.0, 150.0, 45.0), Arc::new(Dielectric::new(1.5))));
    world.push(glass_sphere);

    let metal_sphere = Arc::new(Sphere::new(50.0, Vec3A::new(0.0, 150.0, 145.0), Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 0.0))));
    world.push(metal_sphere);

    // Medium spheres
    let boundary = Arc::new(Sphere::new(70.0, Vec3A::new(360.0, 150.0, 145.0), Arc::new(Dielectric::new(1.5))));
    world.push(boundary.clone());
    world.push(Arc::new(Medium::new(
        boundary,
        0.2,
        Arc::new(Isotropic::new(Color::new(0.2, 0.4, 0.9))),
    )));

    let boundary_large = Arc::new(Sphere::new(5000.0, Vec3A::new(0.0, 0.0, 0.0), Arc::new(Dielectric::new(1.5))));
    world.push(Arc::new(Medium::new(
        boundary_large,
        0.0001,
        Arc::new(Isotropic::new(Color::new(1.0, 1.0, 1.0))),
    )));

    // Textured sphere
    let emat = Arc::new(Lambertian { albedo: Arc::new(texture::Image::new("textures/earthmap.jpg"))});
    let earth_sphere = Arc::new(Sphere::new(100.0, Vec3A::new(400.0, 200.0, 400.0), emat));
    world.push(earth_sphere);
    
    let mut boxes = Vec::new();
    let mut rng = thread_rng();
    let random = Uniform::new(0.0, 165.0_f32);
    
    // Cloud of smaller spheres
    let ns = 1000;
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    for _ in 0..ns {
        boxes.push(Arc::new(Sphere::new(10.0, Vec3A::new(-100.0, 270.0, 395.0) + Vec3A::new(random.sample(&mut rng), random.sample(&mut rng), random.sample(&mut rng)), white.clone())));
    }
    
    let boxes = boxes.into_iter()
        .map(|b| Arc::new(Transform::new(b.clone(), Quat::from_rotation_y(15.0_f32.to_radians()), b.center(0.))) as Arc<dyn Hittable>);
    
    world.extend(boxes);

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let lookfrom = Vec3A::new(478.0, 278.0, -600.0);
    let lookat = Vec3A::new(278.0, 278.0, 0.0);
    let vup = Vec3A::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;
    let focus_dist = 10.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
        TIME_MIN,
        TIME_MAX,
    );

    Render::new(WIDTH, HEIGHT, SAMPLES, Scene::new(&world, TIME_MIN, TIME_MAX, Color::ZERO), camera)
        .to_image()
        .save("examples/final_scene.png")
}