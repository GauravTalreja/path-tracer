use super::{camera::Camera, color::*, rng::RandomNumberGenerator, scene::Scene};
use image::{Rgb, RgbImage};
use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

pub struct Render {
    width: u32,
    height: u32,
    samples_per_pixel: u64,
    camera: Camera,
    scene: Scene,
    rng: RandomNumberGenerator,
}

impl Render {
    pub fn new(
        width: u32,
        height: u32,
        samples_per_pixel: u64,
        scene: Scene,
        camera: Camera,
    ) -> Self {
        let rng = RandomNumberGenerator::new(camera.time_min, camera.time_max);
        Render {
            width,
            height,
            samples_per_pixel,
            camera,
            scene,
            rng,
        }
    }

    fn get_color(&self, x: u32, y: u32) -> Rgb<u8> {
        to_rgb(
            &((0..self.samples_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    let u = (x as f64 + self.rng.uniform_0_1.sample(&mut rng))
                        / (self.width - 1) as f64;
                    let v = (y as f64 + self.rng.uniform_0_1.sample(&mut rng))
                        / (self.height - 1) as f64;
                    let r = self.camera.get_ray(u, v, &self.rng);
                    self.scene.color(&r, 50, &self.rng, 0.001, f64::INFINITY)
                })
                .sum::<Color>()
                / self.samples_per_pixel as f64),
        )
    }

    pub fn to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        let _ = image
            .enumerate_pixels_mut()
            .par_bridge()
            .progress_count((self.width * self.height) as u64)
            .for_each(|(x, y, pixel)| *pixel = self.get_color(x, y));
        image
    }
}
