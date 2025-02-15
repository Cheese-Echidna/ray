use crate::*;
use rand::random;
use std::f64::consts::PI;
use std::fmt::Debug;

pub(crate) const OBJECT_TOLERANCE: f64 = 0.0001;

pub trait RenderObject: Debug + Sync {
    fn intersects(&self, ray: Ray) -> Vec<Vec3>;
    fn attenuation_colour(&self, impact: Vec3, direction: Vec3) -> LinSrgb;
    fn scatter_ray(&self, impact: Vec3, direction: Vec3) -> Ray;
    fn emission(&self, impact: Vec3, direction: Vec3) -> LinSrgb;
    fn random_point_on_surface(&self) -> Vec3;
    fn includes_point_on_surface(&self, point: Vec3) -> bool;
}
