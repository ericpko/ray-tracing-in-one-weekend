use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    // Camera/Eye coordinate system (non-normalized)
    pub origin: Vec3, // look from
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = f32::to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        // create an orthonormal basis u, v, w, for our camera frame
        let vup = Vec3::new(0., 1., 0.);
        let w = -(look_at - look_from).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn shoot_ray(&self, u: f32, v: f32) -> Ray {
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, dir)
    }
}
