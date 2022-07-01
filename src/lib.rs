#![allow(unused)] // ! mute unused warnings for now
use glam::Vec3;

mod ray;
use rand::Rng;
use ray::Ray;
pub mod camera;
use camera::Camera;
pub mod image;
use image::Image;
pub mod geometry;
use geometry::{
    hittable::{HitRecord, Hittable, HittableList},
    sphere,
};

pub fn render(mut image: Image, camera: Camera, world: HittableList) -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("rendering image...");

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..image.samples_per_pixel {
                let ray = camera.shoot_ray(
                    i as f32 + rand::random::<f32>(),
                    j as f32 + rand::random::<f32>(),
                    image.width as f32,
                    image.height as f32,
                );
                pixel_color += ray_color(ray, &world, image.max_depth);
            }

            let pixel_color = antialiasing(pixel_color, image.samples_per_pixel);
            image.pixels.extend(pixel_color);
        }
    }
    write_ppm(image)?;

    Ok(())
}

fn ray_color(ray: Ray, world: &HittableList, depth: i32) -> Vec3 {
    let mut color = Vec3::new(0., 0., 0.);
    let mut rec = HitRecord::default();

    // if we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 { // color is already set to 0, 0, 0
    } else if world.hit(&ray, 0.001, std::f32::INFINITY, &mut rec) {
        let target_point = rec.point + rec.normal + random_in_unit_sphere();
        let reflection_ray = Ray::new(rec.point, target_point - rec.point);
        color = 0.5 * ray_color(reflection_ray, &world, depth - 1);
    } else {
        let unit_dir = ray.dir.normalize();
        let t = 0.5 * (unit_dir.y + 1.0);
        color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
    }

    return color;
}

fn antialiasing(mut color: Vec3, samples_per_pixel: u32) -> [u8; 3] {
    // Divide the color by the number of samples and gamma-correct for gamma = 2.0
    color /= (samples_per_pixel as f32);
    color.x = color.x.sqrt();
    color.y = color.y.sqrt();
    color.z = color.z.sqrt();

    // Translate each pixel to [0, 255]
    convert_rgb(color)
}

fn convert_rgb(color: Vec3) -> [u8; 3] {
    let r = (255.0 * clamp(color.x, 0.0, 1.0)) as u8;
    let g = (255.0 * clamp(color.y, 0.0, 1.0)) as u8;
    let b = (255.0 * clamp(color.z, 0.0, 1.0)) as u8;
    [r, g, b]
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }
    return x;
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new(-1.0f32, 1.0);
    loop {
        let p = Vec3::new(rng.sample(distr), rng.sample(distr), rng.sample(distr));
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn write_ppm(image: Image) -> std::io::Result<()> {
    let width = image.width;
    let height = image.height;
    let mut image_vec = format!("P6\n{width} {height}\n255\n").as_bytes().to_owned();

    image_vec.extend(image.pixels);
    std::fs::write("image.ppm", image_vec)?;
    Ok(())
}
