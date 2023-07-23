use super::*;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

impl Scene {
    pub fn random(n: i64, rng: RandomNumberGenerator) -> Scene {
        let thread_rng = &mut thread_rng();
        let uniform_0_0_5 = Uniform::new(0., 0.5);
        let uniform_0_5_1 = Uniform::new(0.5, 1.);

        let ground = Arc::new(material::Lambertian {
            albedo: Color::splat(0.5),
        });
        let glass = Arc::new(material::Dielectric {
            refractive_index: 1.5,
        });
        let mat2 = Arc::new(material::Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        });
        let mat3 = Arc::new(material::Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.,
        });
        let mut hittables: Vec<Box<dyn hittable::Hittable>> = vec![
            Box::new(hittable::Sphere::new(
                1000.,
                DVec3::new(0., 1000., 0.),
                ground,
            )),
            Box::new(hittable::Sphere::new(
                1.,
                DVec3::new(0., -1., 0.),
                glass.clone(),
            )),
            Box::new(hittable::Sphere::new(1., DVec3::new(-4., -1., 0.), mat2)),
            Box::new(hittable::Sphere::new(1., DVec3::new(4., -1., 0.), mat3)),
        ];

        for a in -n..n {
            for b in -n..n {
                let center = DVec3::new(
                    a as f64 + 0.9 * rng.uniform_0_1.sample(thread_rng),
                    -0.2,
                    b as f64 + 0.9 * rng.uniform_0_1.sample(thread_rng),
                );
                let random = rng.uniform_0_1.sample(thread_rng);
                if random < 0.8 {
                    let albedo = rng.color() * rng.color();
                    let material = Arc::new(material::Lambertian { albedo });
                    hittables.push(Box::new(hittable::Sphere::new(0.2, center, material)));
                } else if random < 0.95 {
                    let albedo = DVec3::new(
                        uniform_0_5_1.sample(thread_rng),
                        uniform_0_5_1.sample(thread_rng),
                        uniform_0_5_1.sample(thread_rng),
                    );
                    let fuzz = uniform_0_0_5.sample(thread_rng);
                    let material = Arc::new(material::Metal { albedo, fuzz });
                    hittables.push(Box::new(hittable::Sphere::new(0.2, center, material)));
                } else {
                    hittables.push(Box::new(hittable::Sphere::new(0.2, center, glass.clone())));
                }
            }
        }

        Scene {
            hittables,
            time_min: 0.001,
            time_max: f64::MAX,
        }
    }
}
