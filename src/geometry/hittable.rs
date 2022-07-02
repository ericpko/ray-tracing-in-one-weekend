use std::rc::Rc;

use glam::Vec3;

use crate::ray::Ray;

use super::material::{self, Material};

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f32,
        front_face: bool,
        material: &Rc<dyn Material>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            material: Rc::clone(material),
        }
    }

    // pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
    //     self.front_face = ray.dir.dot(outward_normal) < 0.0;
    //     self.normal = if self.front_face {
    //         outward_normal
    //     } else {
    //         -outward_normal
    //     }
    // }
}

// Hittable list type
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        return hit_anything;
    }
}
