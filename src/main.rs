use std::rc::Rc;

use glam::Vec3;
use rand::Rng;
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
    let aspect_ratio: f32 = 3.0 / 2.0;
    let image_width: usize = 800;
    let samples_per_pixel = 200;
    let max_depth = 50;
    let image = Image::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // set up a camera
    let look_from = Vec3::new(13., 2., 3.);
    let look_at = Vec3::new(0., 0., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        20.0,
        image.aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let world = generate_random_scene();

    ray_tracing_in_one_weekend::render(image, camera, world)?;
    Ok(())
}

fn generate_random_scene() -> HittableList {
    let mut world = HittableList::new();

    let mat_ground: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    let mut rng = rand::thread_rng();
    let distr = rand::distributions::Uniform::new(0.5f32, 1.0);
    for i in -11..11 {
        for j in -11..11 {
            let a = i as f32;
            let b = j as f32;
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let mat_sphere: Rc<dyn Material>;
                if choose_mat < 0.8 {
                    // diffuse
                    let r = rng.gen::<f32>() * rng.gen::<f32>();
                    let g = rng.gen::<f32>() * rng.gen::<f32>();
                    let b = rng.gen::<f32>() * rng.gen::<f32>();
                    let albedo = Vec3::new(r, g, b);
                    mat_sphere = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::new(rng.sample(distr), rng.sample(distr), rng.sample(distr));
                    let fuzz: f32 = rng.gen_range(0.0..0.5);
                    mat_sphere = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    mat_sphere = Rc::new(Dielectric::new(1.5));
                }
                world.add(Box::new(Sphere::new(center, 0.2, mat_sphere)));
            }
        }
    }

    let mat1: Rc<dyn Material> = Rc::new(Dielectric::new(1.5));
    let mat2: Rc<dyn Material> = Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    let mat3: Rc<dyn Material> = Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));

    world.add(Box::new(Sphere::new(Vec3::new(0., 1., 0.), 1., mat1)));
    world.add(Box::new(Sphere::new(Vec3::new(-4., 1., 0.), 1., mat2)));
    world.add(Box::new(Sphere::new(Vec3::new(4., 1., 0.), 1., mat3)));

    world
}
