use super::prelude::*;

#[derive(Clone)]
pub struct Quad {
    q: Vec3A,
    u: Vec3A,
    v: Vec3A,
    normal: Vec3A,
    d: f32,
    material: Arc<dyn Material>,
}

impl Quad {
    pub fn new(q: Vec3A, u: Vec3A, v: Vec3A, material: Arc<dyn Material>) -> Self {
        let normal = (u.cross(v)).normalize();
        let d = normal.dot(q);

        Quad {
            q,
            u,
            v,
            normal,
            d,
            material,
        }
    }

    pub fn new_box(a: Vec3A, b: Vec3A, material: Arc<dyn Material>) -> Vec<Quad> {
        let min = Vec3A::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
        let max = Vec3A::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

        let dx = Vec3A::new(max.x - min.x, 0.0, 0.0);
        let dy = Vec3A::new(0.0, max.y - min.y, 0.0);
        let dz = Vec3A::new(0.0, 0.0, max.z - min.z);

        vec![
            // Front (XY plane)
            Quad::new(min, dx, dy, material.clone()),
            // Right (YZ plane)
            Quad::new(Vec3A::new(max.x, min.y, min.z), dy, dz, material.clone()),
            // Back (XY plane)
            Quad::new(Vec3A::new(min.x, min.y, min.z), dx, dy, material.clone()),
            // Left (YZ plane)
            Quad::new(min, dy, dz, material.clone()),
            // Top (XZ plane)
            Quad::new(Vec3A::new(min.x, max.y, min.z), dx, dz, material.clone()),
            // Bottom (XZ plane)
            Quad::new(min, dx, dz, material.clone()),
        ]
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitResult> {
        let denom = self.normal.dot(ray.direction());
        if denom.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin())) / denom;
        if t < time_min || t > time_max {
            return None;
        }

        let p = ray.origin() + t * ray.direction();
        let qp = p - self.q;

        let u_projection = qp.dot(self.u);
        let v_projection = qp.dot(self.v);
        let u_length_squared = self.u.length_squared();
        let v_length_squared = self.v.length_squared();

        if u_projection < 0.0 || u_projection > u_length_squared || v_projection < 0.0 || v_projection > v_length_squared {
            return None;
        }

        let normal = if self.normal.dot(ray.direction()) < 0.0 {
            self.normal
        } else {
            -self.normal
        };

        Some(HitResult {
            point: p,
            normal,
            time: t,
            u: u_projection / u_length_squared,
            v: v_projection / v_length_squared,
            material: Arc::downgrade(&self.material),
        })
    }
}

impl Bounded for Quad {
    fn bounding_box(&self, _time_min: f32, _time_max: f32) -> BoundingBox {
        let vertices = [self.q, self.q + self.u, self.q + self.v, self.q + self.u + self.v];
        let mut min = vertices[0];
        let mut max = vertices[0];

        for &vertex in &vertices[1..] {
            min = min.min(vertex);
            max = max.max(vertex);
        }

        BoundingBox { minimum: min, maximum: max }
    }
}

#[derive(Clone)]
pub struct Box {
    sides: Vec<Quad>,
    bb: BoundingBox
}

impl Box {
    pub fn new(min: Vec3A, max: Vec3A, material: Arc<dyn Material>) -> Self {
        Box {
            sides: Quad::new_box(min, max, material),
            bb: BoundingBox { minimum: min, maximum: max }
        }
    }
}

impl Hittable for Box {
    fn hit(&self, ray: &Ray, time_min: f32, time_max: f32) -> Option<HitResult> {
        self.sides.iter()
            .filter_map(|side| side.hit(ray, time_min, time_max))
            .min_by(|hit1, hit2| hit1.time.partial_cmp(&hit2.time).unwrap())
    }
}

impl Bounded for Box {
    fn bounding_box(&self, _: f32, _: f32) -> BoundingBox {
        self.bb
    }
}
