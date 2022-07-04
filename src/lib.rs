use glam::Vec3;

mod ray;
use indicatif::ProgressIterator;
use rand::Rng;
pub mod camera;
use camera::Camera;
pub mod image;
use image::Image;
pub mod geometry;
use geometry::hittable::HittableList;

pub fn render(mut image: Image, camera: Camera, world: HittableList) -> anyhow::Result<()> {
    println!("rendering image...");

    for j in (0..image.height).rev().progress() {
        for i in 0..image.width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..image.samples_per_pixel {
                let mut rng = rand::thread_rng();
                let u = (i as f32 + rng.gen::<f32>()) / (image.width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (image.height as f32 - 1.0);
                let ray = camera.shoot_ray(u, v);
                pixel_color += ray.color(&world, image.max_depth);
            }

            let pixel_color = antialiasing(pixel_color, image.samples_per_pixel);
            image.pixels.extend(pixel_color);
        }
    }
    image.write_ppm()?;

    Ok(())
}

fn antialiasing(mut color: Vec3, samples_per_pixel: u32) -> [u8; 3] {
    // Divide the color by the number of samples and gamma-correct for gamma = 2.0
    color /= samples_per_pixel as f32;
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

fn _random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        // in the same hemisphere as the normal
        return in_unit_sphere;
    } else {
        return -in_unit_sphere;
    }
}

fn random_unit_vector() -> Vec3 {
    random_in_unit_sphere().normalize()
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

fn random_in_unit_disc() -> Vec3 {
    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new(-1.0f32, 1.0);
    loop {
        let p = Vec3::new(rng.sample(distr), rng.sample(distr), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn near_zero(v: &Vec3) -> bool {
    let s: f32 = 1e-8;
    return v.x.abs() < s && v.y.abs() < s && v.z.abs() < s;
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

fn refract(uv: Vec3, n: Vec3, eta_over_etaprime: f32) -> Vec3 {
    let cos_theta = f32::min(-uv.dot(n), 1.0);
    let r_out_perp = eta_over_etaprime * (uv + cos_theta * n);
    let r_out_parallel = -f32::sqrt(f32::abs(1.0 - r_out_perp.length_squared())) * n;
    return r_out_perp + r_out_parallel;
}
