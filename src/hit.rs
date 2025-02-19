use crate::Ray;
use glam::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub ray: Ray,
    pub impact: Vec3,
    pub normal: Vec3,
    // pub uv: Vec2,
}
