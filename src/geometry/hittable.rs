use glam::Vec3;

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32,
}

pub trait Hittable {
    fn hit(&self, t_min: f32, t_max: f32, rec: HitRecord) -> bool;
}
