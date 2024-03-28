use crate::{Color, material::Isotropic};
use super::prelude::*;

pub struct Medium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Arc<dyn Material>,
}

impl Medium {
    pub fn new(boundary: Arc<dyn Hittable>, d: f32, a: Arc<dyn Material>) -> Self {
        Medium {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function: a,
        }
    }

    pub fn new_with_color(boundary: Arc<dyn Hittable>, d: f32, c: Color) -> Self {
        let phase_function = Arc::new(Isotropic::new(c));
        Medium {
            boundary,
            neg_inv_density: -1.0 / d,
            phase_function,
        }
    }
}

impl Hittable for Medium {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitResult> {
        let mut rng = rand::thread_rng();

        let mut rec1 = self.boundary.hit(ray, f32::NEG_INFINITY, f32::INFINITY)?;
        let mut rec2 = self.boundary.hit(ray, rec1.time + 0.0001, f32::INFINITY)?;

        if rec1.time < time_min {
            rec1.time = time_min;
        }
        if rec2.time > time_max {
            rec2.time = time_max;
        }

        if rec1.time >= rec2.time {
            return None;
        }

        if rec1.time < 0.0 {
            rec1.time = 0.0;
        }

        let ray_length = ray.direction().length();
        let distance_inside_boundary = (rec2.time - rec1.time) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f32>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let hit_time = rec1.time + hit_distance / ray_length;
        let hit_point = ray.at(hit_time);

        Some(HitResult {
            time: hit_time,
            point: hit_point,
            normal: Vec3A::new(1.0, 0.0, 0.0),
            material: Arc::downgrade(&self.phase_function),
            u: 0.0,
            v: 0.0,
        })
    }
}
impl Bounded for Medium {
    fn bounding_box(&self, time_min: f32, time_max: f32) -> BoundingBox {
        self.boundary.bounding_box(time_min, time_max)
    }
}