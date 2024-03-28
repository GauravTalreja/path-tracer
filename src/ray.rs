use super::material::Material;
use crate::Vec3A;
use std::sync::Weak;

pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
    time: f32,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Vec3A { self.origin }

    pub fn direction(&self) -> Vec3A {
        self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}

pub struct HitResult {
    pub normal: Vec3A,
    pub time: f32,
    pub u: f32,
    pub v: f32,
    pub point: Vec3A,
    pub material: Weak<dyn Material>,
}
