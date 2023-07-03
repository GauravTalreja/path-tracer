use super::impl_prelude::*;

pub struct Sphere {
    radius: f64,
    center: DVec3,
}

impl Sphere {
    pub fn new(radius: f64, center: DVec3) -> Self {
        Sphere { radius, center }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
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
        if !ray.exists_at(time) {
            time = (-half_b + discriminant_sqrt) / a;
            if !ray.exists_at(time) {
                return None;
            }
        }
        let normal = (ray.at_unchecked(time) - self.center) / self.radius;
        Some(HitResult { normal, time })
    }
}
