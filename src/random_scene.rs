use super::*;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

impl Scene {
    pub fn random_spheres(n: i64, rng: RandomNumberGenerator) -> Scene {
        let thread_rng = &mut thread_rng();
        let uniform_0_0_5 = Uniform::new(0., 0.5);
        let uniform_0_5_1 = Uniform::new(0.5, 1.);

        let ground = Arc::new(material::Lambertian::new(Color::splat(0.5)));
        let glass = Arc::new(material::Dielectric {
            refractive_index: 1.5,
        });
        let mat2 = Arc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        let mat3 = Arc::new(material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        });
        let mut hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
            Arc::new(hittable::Sphere::new(
                1000.,
                Vec3A::new(0., -1000., 0.),
                ground,
            )),
            Arc::new(hittable::Sphere::new(
                1.,
                Vec3A::new(0., 1., 0.),
                glass.clone(),
            )),
            Arc::new(hittable::Sphere::new(1., Vec3A::new(-4., 1., 0.), mat2)),
            Arc::new(hittable::Sphere::new(1., Vec3A::new(4., 1., 0.), mat3)),
        ];

        for a in -n..n {
            for b in -n..n {
                let center = Vec3A::new(
                    a as f32 + 0.9 * rng.uniform_0_1.sample(thread_rng),
                    0.2,
                    b as f32 + 0.9 * rng.uniform_0_1.sample(thread_rng),
                );
                let random = rng.uniform_0_1.sample(thread_rng);
                if random < 0.8 {
                    let color = rng.color() * rng.color();
                    let material = Arc::new(material::Lambertian::new(color));
                    hittables.push(Arc::new(hittable::Sphere::new(0.2, center, material)));
                } else if random < 0.95 {
                    let albedo = Vec3A::new(
                        uniform_0_5_1.sample(thread_rng),
                        uniform_0_5_1.sample(thread_rng),
                        uniform_0_5_1.sample(thread_rng),
                    );
                    let fuzz = uniform_0_0_5.sample(thread_rng);
                    let material = Arc::new(material::Metal { albedo, fuzz });
                    hittables.push(Arc::new(hittable::Sphere::new(0.2, center, material)));
                } else {
                    hittables.push(Arc::new(hittable::Sphere::new(0.2, center, glass.clone())));
                }
            }
        }

        Scene::new(&hittables, 0., 1., Color::new(0.70, 0.80, 1.00))
    }

    pub fn random_moving_spheres(
        n: i64,
        rng: RandomNumberGenerator,
        time_min: f32,
        time_max: f32,
    ) -> Scene {
        let thread_rng = &mut thread_rng();
        let uniform_0_0_5 = Uniform::new(0., 0.5);
        let uniform_0_5_1 = Uniform::new(0.5, 1.);

        let ground = Arc::new(material::Lambertian::new(Color::splat(0.5)));
        let glass = Arc::new(material::Dielectric {
            refractive_index: 1.5,
        });
        let mat2 = Arc::new(material::Lambertian::new(Color::new(0.4, 0.2, 0.1)));
        let mat3 = Arc::new(material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        });
        let mut hittables: Vec<Arc<dyn hittable::Hittable>> = vec![
            Arc::new(hittable::Sphere::new(
                1000.,
                Vec3A::new(0., -1000., 0.),
                ground,
            )),
            Arc::new(hittable::Sphere::new(
                1.,
                Vec3A::new(0., 1., 0.),
                glass.clone(),
            )),
            Arc::new(hittable::Sphere::new(1., Vec3A::new(-4., 1., 0.), mat2)),
            Arc::new(hittable::Sphere::new(1., Vec3A::new(4., 1., 0.), mat3)),
        ];

        for a in -n..n {
            for b in -n..n {
                let center = Vec3A::new(
                    a as f32 + 0.9 * rng.uniform_0_1.sample(thread_rng),
                    0.2,
                    b as f32 + 0.9 * rng.uniform_0_1.sample(thread_rng),
                );
                let random = rng.uniform_0_1.sample(thread_rng);
                if random < 0.8 {
                    let color = rng.color() * rng.color();
                    let material = Arc::new(material::Lambertian::new(color));
                    let center_final =
                        center + Vec3A::new(0., uniform_0_0_5.sample(thread_rng), 0.);
                    hittables.push(Arc::new(hittable::Sphere::new_moving(
                        0.2,
                        center,
                        material,
                        time_min,
                        time_max,
                        center_final,
                    )));
                } else if random < 0.95 {
                    let albedo = Vec3A::new(
                        uniform_0_5_1.sample(thread_rng),
                        uniform_0_5_1.sample(thread_rng),
                        uniform_0_5_1.sample(thread_rng),
                    );
                    let fuzz = uniform_0_0_5.sample(thread_rng);
                    let material = Arc::new(material::Metal { albedo, fuzz });
                    hittables.push(Arc::new(hittable::Sphere::new(0.2, center, material)));
                } else {
                    hittables.push(Arc::new(hittable::Sphere::new(0.2, center, glass.clone())));
                }
            }
        }

        Scene::new(&hittables, time_min, time_max, Color::new(0.70, 0.80, 1.00))
    }
}
