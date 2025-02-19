use std::fmt::Debug;
use crate::Vec3;
use crate::Vec2;
use crate::Ray;

pub(crate) const OBJECT_TOLERANCE: f32 = 0.0001;

pub trait RenderIntersection: Debug + Sync {
    fn intersects(&self, ray: Ray) -> Vec<Vec3>;
    fn normal_at(&self, impact: Vec3) -> Vec3;
    fn ray_normal_closeness(&self, impact: Vec3, direction: Vec3) -> f32 {
        (-direction).normalize().dot(self.normal_at(impact)).abs()
    }
    fn random_point_on_surface(&self) -> Vec3;
    fn includes_point_on_surface(&self, point: Vec3) -> bool;
    fn uv(&self, at: Vec3) -> Vec2;
}
