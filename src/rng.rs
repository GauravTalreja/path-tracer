use glam::DVec3;
use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng, thread_rng};

use crate::Color;

pub struct RandomNumberGenerator {
    pub uniform_0_1: Uniform<f64>,
    pub uniform_minus_1_1: Uniform<f64>,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            uniform_0_1: Uniform::new(0., 1.),
            uniform_minus_1_1: Uniform::new(-1., 1.),
        }
    }
    pub fn in_unit_cube(&self) -> DVec3 {
        let mut rng = thread_rng();
        DVec3 {
            x: self.uniform_minus_1_1.sample(&mut rng),
            y: self.uniform_minus_1_1.sample(&mut rng),
            z: self.uniform_minus_1_1.sample(&mut rng),
        }
    }

    pub fn in_unit_sphere(&self) -> DVec3 {
        let mut random = self.in_unit_cube();
        while random.length_squared() >= 1. {
            random = self.in_unit_cube();
        }
        random
    }

    pub fn in_hemishphere(&self, normal: &DVec3) -> DVec3 {
        let in_unit_sphere = self.in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    fn try_in_unit_disk(&self, rng: &mut ThreadRng) -> DVec3 {
        DVec3 {
            x: self.uniform_minus_1_1.sample(rng),
            y: self.uniform_minus_1_1.sample(rng),
            z: 0.,
        }
    }

    pub fn in_unit_disk(&self) -> DVec3 {
        let mut rng = thread_rng();
        let mut random = self.try_in_unit_disk(&mut rng);
        while random.length_squared() >= 1. {
            random = self.try_in_unit_disk(&mut rng);
        }
        random
    }

    pub fn color(&self) -> Color {
        let mut rng = thread_rng();
        Color::new(
            self.uniform_0_1.sample(&mut rng),
            self.uniform_0_1.sample(&mut rng),
            self.uniform_0_1.sample(&mut rng),
        )
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        RandomNumberGenerator::new()
    }
}
