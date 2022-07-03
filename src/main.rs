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
    let camera = Camera::new(
        image.aspect_ratio,
        2.,
        1.,
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 0., -1.),
    );
    // create some materials
    let mat_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let mat_center: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    // let mat_left: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3));
    let mat_left: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let mat_right: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0));

    // set up objects in the world
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Rc::clone(&mat_ground),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Rc::clone(&mat_center),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&mat_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(-1.0, 0.0, -1.0),
        -0.4,
        Rc::clone(&mat_left),
    )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Rc::clone(&mat_right),
    )));

    ray_tracing_in_one_weekend::render(image, camera, world)?;
    Ok(())
}
