use super::prelude::*;
use std::f32::consts::PI;

pub struct Sphere {
    radius: f32,
    center: Vec3A,
    material: Arc<dyn Material>,
    moving_sphere: Option<MovingSphere>,
}

pub struct MovingSphere {
    pub time_min: f32,
    pub center: Vec3A,
    pub time_max: f32,
}

impl Sphere {
    pub fn new(radius: f32, center: Vec3A, material: Arc<dyn Material>) -> Self {
        Sphere {
            radius,
            center,
            material,
            moving_sphere: None,
        }
    }

    pub fn new_moving(
        radius: f32,
        center: Vec3A,
        material: Arc<dyn Material>,
        time_min: f32,
        time_max: f32,
        center_final: Vec3A,
    ) -> Self {
        Sphere {
            radius,
            center,
            material,
            moving_sphere: Some(MovingSphere {
                time_min,
                center: center_final,
                time_max,
            }),
        }
    }

    pub fn center(&self, time: f32) -> Vec3A {
        match self.moving_sphere {
            Some(MovingSphere {
                time_min,
                center,
                time_max,
            }) => self
                .center
                .lerp(center, (time - time_min) / (time_max - time_min)),
            None => self.center,
        }
    }

    pub fn get_uv(point: &Vec3A) -> (f32, f32) {
        let phi = (-point.z).atan2(point.x) + PI;
        let theta = (-point.y).acos();
        let u = phi / (2. * PI);
        let v = theta / PI;
        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitResult> {
        let center = self.center(ray.time());
        let oc = ray.origin() - center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
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
        let normal: Vec3A = (point - center) / self.radius;
        let (u, v) = Sphere::get_uv(&normal);
        Some(HitResult {
            normal,
            time,
            point,
            material: Arc::downgrade(&self.material),
            u,
            v,
        })
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self, time_min: f32, time_max: f32) -> BoundingBox {
        match self.moving_sphere {
            Some(_) => BoundingBox::surrounding(&[
                &bounding_box(self.center(time_min), self.radius),
                &bounding_box(self.center(time_max), self.radius),
            ]),
            None => bounding_box(self.center, self.radius),
        }
    }
}

fn bounding_box(center: Vec3A, radius: f32) -> BoundingBox {
    BoundingBox {
        minimum: center - Vec3A::splat(radius),
        maximum: center + Vec3A::splat(radius),
    }
}
