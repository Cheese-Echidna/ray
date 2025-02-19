use crate::*;
use rand::random;
use std::f32::consts::PI;
use std::fmt::Debug;
use palette::encoding::Linear;
use palette::rgb::Rgb;
use crate::objects::material::RenderMaterial;
use crate::utils::{bounce_across_normal, random_cosine_direction};

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

#[derive(Debug)]
pub struct RenderObject {
    inter_function: Box<dyn RenderIntersection>,
    material: RenderMaterial
}

impl RenderObject {
    pub fn light_emitted_at_point_in_direction(&self, impact: Vec3, direction: Vec3) -> Vec3 {
        self.material.emissivity * self.inter_function.ray_normal_closeness(impact, direction) as f32
    }
    pub fn is_emitter(&self) -> bool {
        self.material.emissivity != BLACK.to_vec3()
    }
    pub fn emission(&self) -> Vec3 {
        self.material.emissivity
    }
    pub fn colour(&self) -> Vec3 {
        self.material.albedo
    }
    fn scatter_ray(&self, impact: Vec3, direction: Vec3) -> Option<Ray> {
        if self.material.reflectivity == 0.0 {
            return None
        }
        let normal = utils::fix_normal(direction, self.normal_at(impact));
        let reflect_dir = bounce_across_normal(direction, normal);

        let random_hemi_dir = reflect_dir + self.material.roughness * random_cosine_direction(normal);

        Some(Ray::new(impact, random_hemi_dir.normalize()))
    }
}

impl RenderIntersection for RenderObject {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.inter_function.intersects(ray)
    }

    fn normal_at(&self, impact: Vec3) -> Vec3 {
        self.inter_function.normal_at(impact)
    }

    fn random_point_on_surface(&self) -> Vec3 {
        self.inter_function.random_point_on_surface()
    }

    fn includes_point_on_surface(&self, point: Vec3) -> bool {
        self.inter_function.includes_point_on_surface(point)
    }

    fn uv(&self, at: Vec3) -> Vec2 {
        self.inter_function.uv(at)
    }
}