use glam::Vec3;
use ray_tracing_iow::{camera::Camera, image::Image};

pub fn main() -> anyhow::Result<()> {
    // set up the image dimensions in pixels
    let image = Image::new(16.0 / 9.0, 400);
    // set up a camera
    let camera = Camera::new(
        image.aspect_ratio,
        2.,
        1.,
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., -1.),
    );
    ray_tracing_iow::render(image, camera)?;
    Ok(())
}
