#![allow(unused)] // ! mute unused warnings for now
use glam::Vec3;

mod ray;
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
            let ray = camera.shoot_ray(i as f32, j as f32, image.width as f32, image.height as f32);
            let pixel_color = ray_color(ray, &world);

            image.pixels.extend(pixel_color);
        }
    }
    write_ppm(image)?;

    Ok(())
}

fn ray_color(ray: Ray, world: &HittableList) -> [u8; 3] {
    let mut rec = HitRecord::default();
    if world.hit(&ray, 0., std::f32::INFINITY, &mut rec) {
        let color = 0.5 * (rec.normal + Vec3::new(1., 1., 1.));
        return convert_rgb(color);
    }
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    let color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
    return convert_rgb(color);
}

fn convert_rgb(color: Vec3) -> [u8; 3] {
    let r = (color.x * 255.999) as u8;
    let g = (color.y * 255.999) as u8;
    let b = (color.z * 255.999) as u8;
    [r, g, b]
}

fn write_ppm(image: Image) -> std::io::Result<()> {
    let width = image.width;
    let height = image.height;
    let mut image_vec = format!("P6\n{width} {height}\n255\n").as_bytes().to_owned();

    image_vec.extend(image.pixels);
    std::fs::write("image.ppm", image_vec)?;
    Ok(())
}
