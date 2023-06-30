use std::ops::Sub;

use glam::DVec3;

use crate::ray::{HitResult, Ray};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray) -> Option<HitResult>;
}

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
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let d = half_b * half_b - a * c;
        match d < 0. {
            true => None,
            false => {
                let t = (-half_b - d.sqrt()) / a;
                let normal = DVec3::normalize(ray.at(t).sub(self.center));
                Some(HitResult { normal })
            }
        }
    }
}
