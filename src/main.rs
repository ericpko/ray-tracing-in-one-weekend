use std::rc::Rc;

use glam::Vec3;
use ray_tracing_in_one_weekend::{
    camera::Camera,
    geometry::{
        hittable::HittableList,
        material::{Dielectric, Lambertian, Material, Metal},
        sphere::Sphere,
    },
    image::Image,
};

pub fn main() -> anyhow::Result<()> {
    // set up the image dimensions in pixels
    let image = Image::new(16.0 / 9.0, 400, 100, 50);
    // set up a camera
    let camera = Camera::new(90.0, image.aspect_ratio, 1.0, Vec3::new(0.0, 0.0, 0.0));

    // create some materials (Rc = reference counting pointer = shared pointers)
    // let mat_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    // let mat_center: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let mat_left: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    let mat_right: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));

    // set up objects in the world
    let r = f32::cos(std::f32::consts::PI / 4.0);
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(-r, 0.0, -1.0),
        r,
        Rc::clone(&mat_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(r, 0.0, -1.0),
        r,
        Rc::clone(&mat_right),
    )));

    ray_tracing_in_one_weekend::render(image, camera, world)?;
    Ok(())
}
