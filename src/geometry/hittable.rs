use glam::Vec3;

use crate::ray::Ray;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

// NOTE not sure about instantiating HitRecord or having set_face_normal function
impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.0,
            front_face: false,
        }
    }

    // NOTE change ray: &Ray signature? Need to be borrowed?
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

// FIXME might have to change rec: &mut HitRecord
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32, rec: HitRecord) -> bool;
}
