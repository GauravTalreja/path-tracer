use super::impl_prelude::*;

pub struct Sphere {
    radius: f64,
    center: DVec3,
    material: Arc<dyn Material>,
    moving_sphere: Option<MovingSphere>,
}

pub struct MovingSphere {
    pub time_min: f64,
    pub center: DVec3,
    pub time_max: f64,
}

impl Sphere {
    pub fn new(radius: f64, center: DVec3, material: Arc<dyn Material>) -> Self {
        Sphere {
            radius,
            center,
            material,
            moving_sphere: None,
        }
    }

    pub fn new_moving(
        radius: f64,
        center: DVec3,
        material: Arc<dyn Material>,
        time_min: f64,
        time_max: f64,
        center_final: DVec3,
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

    pub fn center(&self, time: f64) -> DVec3 {
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
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        let center = self.center(ray.time());
        let oc = *ray.origin() - center;
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
        let normal = (point - center) / self.radius;
        Some(HitResult {
            normal,
            time,
            point,
            material: Arc::downgrade(&self.material),
        })
    }
}

impl Bounded for Sphere {
    fn bounding_box(&self, time_min: f64, time_max: f64) -> BoundingBox {
        match self.moving_sphere {
            Some(_) => BoundingBox::surrounding(&[
                &bounding_box(self.center(time_min), self.radius),
                &bounding_box(self.center(time_max), self.radius),
            ]),
            None => bounding_box(self.center, self.radius),
        }
    }
}

fn bounding_box(center: DVec3, radius: f64) -> BoundingBox {
    BoundingBox {
        minimum: center - DVec3::splat(radius),
        maximum: center + DVec3::splat(radius),
    }
}
