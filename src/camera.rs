use glam::Vec3;

use crate::ray::Ray;

pub struct Camera {
    // Camera constants
    pub aspect_ratio: f32,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
    pub lower_left_corner: Vec3,

    // Camera/Eye coordinate system
    pub origin: Vec3, // eye
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        viewport_height: f32,
        focal_length: f32,
        origin: Vec3,
        look_at: Vec3,
    ) -> Self {
        let viewport_width = aspect_ratio * viewport_height;

        // https://learnopengl.com/Getting-started/Camera
        let cam_dir = -(look_at - origin).normalize(); // this is actually the reverse direction of what it's looking at
        let up = Vec3::new(0., 1., 0.);
        let right = up.cross(cam_dir).normalize();
        let up = cam_dir.cross(right);

        // calculate the lower left corner of the viewport
        let lower_left_corner = origin
            - right * viewport_width / 2.
            - up * viewport_height / 2.
            - Vec3::new(0., 0., focal_length);

        Self {
            aspect_ratio,
            viewport_width,
            viewport_height,
            focal_length,
            lower_left_corner,
            origin,
            u: right,
            v: up,
            w: cam_dir,
        }
    }

    pub fn shoot_ray(&self, i: f32, j: f32, image_width: f32, image_height: f32) -> Ray {
        // Fundamentals of Computer Graphics -- Section 4.3.1
        let l = -self.viewport_width / 2.;
        let b = -self.viewport_height / 2.;
        let u = l + self.viewport_width * i / (image_width - 1.);
        let v = b + self.viewport_height * j / (image_height - 1.);
        let w = -self.focal_length;

        let ray_dir = w * self.w + u * self.u + v * self.v;
        Ray::new(self.origin, ray_dir)
    }
}
