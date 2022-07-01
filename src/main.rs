use glam::Vec3;
use ray_tracing_in_one_weekend::{
    camera::Camera,
    geometry::{hittable::HittableList, sphere::Sphere},
    image::Image,
};

pub fn main() -> anyhow::Result<()> {
    // set up the image dimensions in pixels
    let image = Image::new(16.0 / 9.0, 400, 100, 50);
    // set up a camera
    let camera = Camera::new(
        image.aspect_ratio,
        2.,
        1.,
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., -1.),
    );
    // set up objects in the world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    ray_tracing_in_one_weekend::render(image, camera, world)?;
    Ok(())
}
