use std::fmt::Debug;
use crate::*;

pub trait RenderObject: Debug {
    fn intersects(&self, ray: Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Vec3;
    fn colour(&self) -> Srgb;
}