use glam::Vec3;

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, t_min: f32, t_max: f32, rec: HitRecord) -> bool {
        true
    }
}
