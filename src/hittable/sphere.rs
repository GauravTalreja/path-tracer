use super::impl_prelude::*;

pub struct Sphere {
    radius: f64,
    center: DVec3,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(radius: f64, center: DVec3, material: Arc<dyn Material>) -> Self {
        Sphere {
            radius,
            center,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        let oc = *ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(*ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_sqrt = discriminant.sqrt();
        let mut time = (-half_b - discriminant_sqrt) / a;
        if time < time_min || time_max < time {
            time = (-half_b + discriminant_sqrt) / a;
            if time < time_min || time_max < time {
                return None;
            }
        }
        let point = ray.at(time);
        let normal = (point - self.center) / self.radius;
        Some(HitResult {
            normal,
            time,
            point,
            material: Arc::downgrade(&self.material),
        })
    }
}
