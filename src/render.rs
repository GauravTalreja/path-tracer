use glam::DVec3;
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
    pixel_delta_u : DVec3,
    pixel_delta_v : DVec3,
    pixel00_loc : DVec3,
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
        let rng = RandomNumberGenerator::new();
        let pixel_delta_u = camera.viewport_u / width as f64;
        let pixel_delta_v = camera.viewport_v / height as f64;
        let pixel00_loc = camera.viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        Render {
            width,
            height,
            samples_per_pixel,
            camera,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
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
                    
                    let pixel_center = self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
                    
                    let px = -0.5 + self.rng.uniform_0_1.sample(&mut rng);
                    let py = -0.5 + self.rng.uniform_0_1.sample(&mut rng);
                    let pixel_sample_square = (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
                    
                    let pixel_sample = pixel_center + pixel_sample_square;
                    
                    let r = self.camera.get_ray(pixel_sample, &self.rng);
                    self.scene.color(&r, 50, &self.rng, 0.001, f64::INFINITY)
                })
                .sum::<Color>()
                / self.samples_per_pixel as f64),
        )
    }

    pub fn to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        image
            .enumerate_pixels_mut()
            .par_bridge()
            .progress_count((self.width * self.height) as u64)
            .for_each(|(x, y, pixel)| *pixel = self.get_color(x, y));
        image
    }
}
