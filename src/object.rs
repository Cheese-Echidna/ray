use std::f64::consts::PI;
use std::fmt::Debug;
use rand::random;
use crate::*;

pub trait RenderObject: Debug {
    fn intersects(&self, ray: Ray) -> Vec<Vec3>;
    // fn normal_at(&self, point: Vec3) -> Vec3;
    fn scatter(&self, impact: Vec3, direction: Vec3) -> Option<(LinSrgb, Ray)>;
    fn emission(&self, impact: Vec3, direction: Vec3) -> LinSrgb;
}
