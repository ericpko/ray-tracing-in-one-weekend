#![allow(unused)] // ! mute unused warnings for now
use glam::Vec3;

mod ray;
use ray::Ray;

// Image dimensions in pixels
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as usize; // 225

// Camera constants
const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.0;

pub fn render() -> anyhow::Result<()> {
    pretty_env_logger::init();
    log::info!("rendering image...");

    // Camera/Eye coordinate system
    let origin = Vec3::new(0., 0., 0.);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH);

    let mut image: Vec<u8> = Vec::with_capacity(3 * IMAGE_WIDTH * IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = i as f32 / (IMAGE_WIDTH as f32 - 1.);
            let v = j as f32 / (IMAGE_HEIGHT as f32 - 1.);
            let ray_dir = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(origin, ray_dir);
            let pixel_color = ray_color(ray);

            image.extend(pixel_color);
        }
    }
    write_ppm(image)?;

    Ok(())
}

fn ray_color(ray: Ray) -> Vec<u8> {
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

fn write_ppm(image: Vec<u8>) -> std::io::Result<()> {
    let mut image_vec = format!("P6\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n")
        .as_bytes()
        .to_owned(); // &[u8] -> Vec<u8>

    image_vec.extend(image);
    std::fs::write("image.ppm", image_vec)?;
    Ok(())
}
