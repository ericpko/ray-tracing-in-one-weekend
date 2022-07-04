use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

use glam::Vec3;

use crate::ray::Ray;

use super::material::{self, Material};

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        normal: Vec3,
        t: f32,
        front_face: bool,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

// Hittable list type
pub struct HittableList {
    // objects: Vec<Arc<dyn Hittable>>,
    objects: Arc<Mutex<Vec<Arc<dyn Hittable>>>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn add(&mut self, obj: Arc<dyn Hittable>) {
        // self.objects.push(obj);
        self.objects.lock().unwrap().push(obj);
    }

    pub fn clear(&mut self) {
        // self.objects.clear();
        self.objects.lock().unwrap().clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        // for obj in self.objects.iter() {
        //     if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
        //         closest_so_far = hit.t;
        //         hit_anything = Some(hit);
        //     }
        // }
        for obj in self.objects.lock().unwrap().iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }

        return hit_anything;
    }
}
