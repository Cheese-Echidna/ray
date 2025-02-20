use crate::hit::Hit;
use crate::Ray;
use crate::Vec2;
use crate::Vec3;
use std::fmt::Debug;

pub(crate) const OBJECT_TOLERANCE: f32 = 0.0001;

pub trait RenderIntersection: Debug + Sync {
    fn intersects(&self, ray: Ray) -> Vec<Vec3>;
    fn normal_at(&self, impact: Vec3) -> Vec3;
    fn includes_point_on_surface(&self, point: Vec3) -> bool;
    fn uv(&self, at: Vec3) -> Vec2;
}

