use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    // Camera constants
    pub vfov: f32, // vertical field-of-view in degrees
    pub aspect_ratio: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,

    // Camera/Eye coordinate system
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect_ratio: f32, focal_length: f32, origin: Vec3) -> Self {
        let theta = f32::to_radians(vfov);
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0., focal_length);

        Self {
            vfov,
            aspect_ratio,
            viewport_width,
            viewport_height,
            focal_length,
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
