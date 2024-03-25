use super::prelude::*;

pub struct Rectangle {
    a0: f64,
    a1: f64,
    b0: f64,
    b1: f64,
    k: f64,
    material: Arc<dyn Material>,
    a_idx: usize,
    b_idx: usize,
    k_idx: usize,
}

pub enum Plane {
    XY,
    YZ,
    ZX,
}

impl Rectangle {
    pub fn new(
        a0: f64,
        a1: f64,
        b0: f64,
        b1: f64,
        k: f64,
        material: Arc<dyn Material>,
        plane: Plane,
    ) -> Self {
        let (a_idx, b_idx, k_idx) = match plane {
            Plane::XY => (0, 1, 2),
            Plane::YZ => (1, 2, 0),
            Plane::ZX => (2, 0, 1),
        };
        Self {
            a0,
            a1,
            b0,
            b1,
            k,
            material,
            a_idx,
            b_idx,
            k_idx,
        }
    }
}

impl Bounded for Rectangle {
    fn bounding_box(&self, _time_min: f64, _time_max: f64) -> BoundingBox {
        let mut minimum: [f64; 3] = [0.; 3];
        minimum[self.a_idx] = self.a0;
        minimum[self.b_idx] = self.b0;
        minimum[self.k_idx] = self.k - 0.0001;
        let minimum = DVec3::from_array(minimum);

        let mut maximum: [f64; 3] = [0.; 3];
        maximum[self.a_idx] = self.a1;
        maximum[self.b_idx] = self.b1;
        maximum[self.k_idx] = self.k + 0.0001;
        let maximum = DVec3::from_array(maximum);

        BoundingBox { minimum, maximum }
    }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &Ray, time_min: f64, time_max: f64) -> Option<HitResult> {
        let time = (self.k - ray.origin()[self.k_idx]) / ray.direction()[self.k_idx];
        if time < time_min || time > time_max {
            return None;
        }
        let a = ray.origin()[self.a_idx] + time * ray.direction()[self.a_idx];
        let b = ray.origin()[self.b_idx] + time * ray.direction()[self.b_idx];
        if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
            return None;
        }
        let mut normal = DVec3::ZERO;
        normal[self.k_idx] = 1.;
        if ray.direction().dot(normal) >= 0. {
            normal = -normal;
        }
        let u = (a - self.a0) / (self.a1 - self.a0);
        let v = (b - self.b0) / (self.b1 - self.b0);
        let point = ray.at(time);
        let material = Arc::downgrade(&self.material);
        Some(HitResult {
            normal,
            time,
            u,
            v,
            point,
            material,
        })
    }
}
