use glam::Vec3;

use crate::geometry::hittable::{Hittable, HittableList};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            dir: direction,
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.dir
    }

    pub fn color(&self, world: &HittableList, depth: i32) -> Vec3 {
        let mut color = Vec3::new(0., 0., 0.);

        // if we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 { // color is already set to 0, 0, 0
        } else if let Some(hit_rec) = world.hit(self, 0.001, f32::MAX) {
            if let Some((scattered, attenuation)) = hit_rec.material.scatter(self, &hit_rec) {
                color = attenuation * scattered.color(world, depth - 1);
            }
        } else {
            let unit_dir = self.dir.normalize();
            let t = 0.5 * (unit_dir.y + 1.0);
            color = (1. - t) * Vec3::new(1., 1., 1.) + t * Vec3::new(0.5, 0.7, 1.0);
        }

        return color;
    }
}
