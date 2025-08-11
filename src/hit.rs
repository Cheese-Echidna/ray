use crate::objects::RenderObject;
use crate::utils::vec_format;
use crate::{utils, Ray};
use glam::{Vec2, Vec3};
use std::fmt::Formatter;

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub ray: Ray,
    pub impact: Vec3,
    pub normal: Vec3,
    pub original_normal: Vec3,
    pub uv: Vec2,
    pub uv_derivatives: (Vec3, Vec3),
}

impl Hit {
    pub fn new(object: &RenderObject, intersection: Vec3, ray: Ray) -> Self {
        let normal = object.intersector.normal_at(intersection).normalize();
        let uv = object.intersector.uv(intersection);

        Hit {
            ray,
            impact: intersection,
            normal: utils::fix_normal(normal, ray.direction()).normalize(),
            original_normal: normal,
            uv,
            uv_derivatives: object.intersector.uv_derivatives(uv),
        }
    }
    pub fn on_outside(&self) -> bool {
        self.normal.dot(self.original_normal) >= 0.
    }
    pub fn direction(&self) -> Vec3 {
        self.ray.direction()
    }
}

impl std::fmt::Display for Hit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Hit(impact: {}, direction: {}, normal: {}, og normal: {}, outside: {})",
            vec_format(self.impact),
            vec_format(self.direction()),
            self.normal,
            self.original_normal,
            self.on_outside(),
        )
    }
}
