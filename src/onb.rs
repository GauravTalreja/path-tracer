use glam::DVec3;

pub struct Onb {
    pub u: DVec3,
    pub v: DVec3,
    pub w: DVec3,
}

impl Onb {
    pub fn local(&self, vec: &DVec3) -> DVec3 {
        vec.x * self.u + vec.y * self.v + vec.z * self.w
    }

    pub fn new_from_w(w: &DVec3) -> Self {
        let a = match w.x.abs() > 0.9 {
            true => DVec3::Y,
            false => DVec3::X,
        };
        let v = w.cross(a).normalize();
        let u = w.cross(v);
        Self { u, v, w: *w }
    }
}
