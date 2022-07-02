use glam::Vec3;

use crate::{near_zero, random_in_unit_sphere, random_unit_vector, ray::Ray, reflect};

use super::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut scatter_dir = rec.normal + random_unit_vector();

        // catch degenerate scatter direction
        if near_zero(&scatter_dir) {
            scatter_dir = rec.normal;
        }

        let scattered = Ray::new(rec.point, scatter_dir);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        let fuzz_radius = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self {
            albedo,
            fuzz: fuzz_radius,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(ray_in.dir.normalize(), rec.normal);
        // let scattered = Ray::new(rec.point, reflected);
        let scattered = Ray::new(rec.point, reflected + self.fuzz * random_in_unit_sphere());
        let attenuation = self.albedo;

        if scattered.dir.dot(rec.normal) > 0.0 {
            return Some((scattered, attenuation));
        }
        None
    }
}
