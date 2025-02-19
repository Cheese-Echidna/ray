use crate::hit::Hit;
use crate::Ray;
use crate::Vec2;
use crate::Vec3;
use std::fmt::Debug;

pub(crate) const OBJECT_TOLERANCE: f32 = 0.0001;

pub trait RenderIntersection: Debug + Sync {
    fn intersects(&self, ray: Ray) -> Vec<Vec3>;
    fn fix_normal(&self, hit: Hit) -> Vec3 {
        let normal = self.normal_at(hit.impact);
        let direction = hit.ray.direction();
        if normal.dot(direction) < 0.0 {
            -normal
        } else {
            normal
        }
    }
    fn normal_at(&self, impact: Vec3) -> Vec3;
    fn includes_point_on_surface(&self, point: Vec3) -> bool;
    fn uv(&self, at: Vec3) -> Vec2;
}

pub fn ray_normal_closeness(hit: Hit) -> f32 {
    (-hit.ray.direction())
        .normalize()
        .dot(hit.normal)
        .abs()
}