use super::{camera::Camera, color::*, rng::RandomNumberGenerator, scene::Scene};
use image::{Rgb, RgbImage};
use rand::{prelude::Distribution, thread_rng};
use rayon::prelude::*;

pub struct Render {
    width: u32,
    height: u32,
    samples: u64,
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
        let rng = RandomNumberGenerator::new();
        Render {
            width,
            height,
            samples: samples_per_pixel,
            camera,
            scene,
            rng,
        }
    }

    fn get_color(&self, x: u32, y: u32) -> Rgb<u8> {
        to_rgb(
            &((0..self.samples)
                .into_par_iter()
                .map(|_| {
                    let mut rng = thread_rng();
                    let u = (x as f64 + self.rng.uniform_0_1.sample(&mut rng))
                        / (self.width - 1) as f64;
                    let v = (y as f64 + self.rng.uniform_0_1.sample(&mut rng))
                        / (self.height - 1) as f64;
                    let r = self.camera.get_ray(u, v);
                    self.scene.color(&r, 50, &self.rng)
                })
                .sum::<Color>()
                / self.samples as f64),
        )
    }

    pub fn to_image(&self) -> RgbImage {
        let mut image = RgbImage::new(self.width, self.height);
        let _ = image
            .enumerate_pixels_mut()
            .par_bridge()
            .for_each(|(x, y, pixel)| *pixel = self.get_color(x, y));
        image
    }
}
