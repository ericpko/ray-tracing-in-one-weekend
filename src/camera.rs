use glam::Vec3;

use crate::{random_in_unit_disc, ray::Ray};

pub struct Camera {
    // Camera/Eye coordinate system
    pub origin: Vec3, // look from
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
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
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            u,
            v,
            w,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
        }
    }

    pub fn shoot_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        let origin = self.origin + offset;
        let dir = self.lower_left_corner + s * self.horizontal + t * self.vertical - origin;
        Ray::new(origin, dir)
    }
}
