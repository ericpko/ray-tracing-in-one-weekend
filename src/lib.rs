#![allow(unused)] // ! mute unused warnings for now
use glam::Vec3;

mod ray;
use ray::Ray;
pub mod camera;
use camera::Camera;
pub mod image;
use image::Image;
mod geometry;

pub fn render(mut image: Image, camera: Camera) -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("rendering image...");

    for j in (0..image.height).rev() {
        for i in 0..image.width {
            let ray = camera.shoot_ray(i as f32, j as f32, image.width as f32, image.height as f32);
            let pixel_color = ray_color(ray);

            image.pixels.extend(pixel_color);
        }
    }
    write_ppm(image)?;

    Ok(())
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.dir.length_squared();
    let half_b = oc.dot(ray.dir);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn ray_color(ray: Ray) -> Vec<u8> {
    let t = hit_sphere(Vec3::new(0., 0., -1.), 0.5, &ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0., 0., -1.)).normalize();
        let color = 0.5 * Vec3::new(n.x + 1., n.y + 1., n.z + 1.);
        return vec![
            (color.x * 255.999) as u8,
            (color.y * 255.999) as u8,
            (color.z * 255.999) as u8,
        ];
    }
    let unit_dir = ray.dir.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    let color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
    let color = vec![
        (color.x * 255.999) as u8,
        (color.y * 255.999) as u8,
        (color.z * 255.999) as u8,
    ];
    color
}

fn write_ppm(image: Image) -> std::io::Result<()> {
    let width = image.width;
    let height = image.height;
    let mut image_vec = format!("P6\n{width} {height}\n255\n").as_bytes().to_owned();

    image_vec.extend(image.pixels);
    std::fs::write("image.ppm", image_vec)?;
    Ok(())
}
