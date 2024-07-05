pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod vec3;

use crate::camera::Camera;
use crate::color::{clamp, ray_color};
use crate::hittable_list::HittableList;
use crate::vec3::Vec3;
use rand::Rng;
use rayon::iter::{IndexedParallelIterator, ParallelIterator};
use rayon::prelude::ParallelSliceMut;
use std::ops::DerefMut;

use image::{Rgb, RgbImage};

const PI: f64 = 3.1415926535897932385;
const INF: f64 = f64::INFINITY;

fn deg_to_rad(deg: f64) -> f64 {
    return deg * PI / 180.;
}

pub struct ImageConfig {
    pub aspect_ratio: f64,
    pub width: u32,
    pub height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl ImageConfig {
    pub fn new() -> ImageConfig {
        ImageConfig {
            aspect_ratio: 16. / 9.,
            width: 400,
            height: (400. / 16. * 9.) as u32,
            samples_per_pixel: 100,
            max_depth: 50,
        }
    }
}

pub fn render(world: HittableList, camera: Camera, config: ImageConfig) -> RgbImage {
    let mut img = RgbImage::new(config.width, config.height);

    let stride = config.width as usize * 3;

    img.deref_mut()
        .par_chunks_exact_mut(stride)
        .enumerate()
        .for_each(|(y, row)| {
            for (x, pixel) in row.chunks_exact_mut(3).enumerate() {
                if x >= config.width as usize || y >= config.height as usize {
                    continue;
                }

                let mut color = Vec3::new(0., 0., 0.);

                for _s in 0..config.samples_per_pixel {
                    let u: f64 =
                        (x as f64 + rand::thread_rng().gen::<f64>()) / (config.width as f64 - 1.);
                    let v: f64 = ((config.height - y as u32) as f64
                        + rand::thread_rng().gen::<f64>())
                        / (config.height as f64 - 1.);

                    let r = camera.get_ray(u, v);
                    color += ray_color(&r, &world, config.max_depth);
                }

                let scale = 1. / config.samples_per_pixel as f64;

                let a = clamp((color.x() * scale).sqrt(), 0., 0.999);
                let b = clamp((color.y() * scale).sqrt(), 0., 0.999);
                let c = clamp((color.z() * scale).sqrt(), 0., 0.999);

                let Rgb(a) = Rgb([(256. * a) as u8, (256. * b) as u8, (256. * c) as u8]);

                pixel.copy_from_slice(&a);
            }
        });

    img
}
