use crate::*;
use rand::random;
use std::f32::consts::PI;
use std::fmt::Debug;
use palette::encoding::Linear;
use palette::rgb::Rgb;
use crate::objects::material::RenderMaterial;
use crate::utils::{bounce_across_normal, compute_fresnel, random_cosine_direction};

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
    pub fn new(b: Box<dyn RenderIntersection>, m: RenderMaterial) -> Self {
        Self {
            inter_function: b,
            material: m,
        }
    }

    pub fn is_emitter(&self) -> bool {
        self.material.emission_colour.length() > 0.0
    }

    pub fn emission(&self) -> Vec3 {
        self.material.emission_colour
    }

    pub fn colour(&self) -> Vec3 {
        self.material.base_colour
    }
    pub fn transmission(&self) -> f32 {
        self.material.transmission
    }
    pub fn metallic(&self) -> f32 {
        self.material.metallic
    }

    pub(crate) fn scatter_ray(&self, impact: Vec3, direction: Vec3, ray_ior: f32) -> Ray {
        let normal = utils::fix_normal(direction, self.normal_at(impact));
        let reflect_dir = bounce_across_normal(direction, normal);

        let random_hemi_dir = reflect_dir + self.material.roughness * random_cosine_direction(normal) * 0.001;

        let reflected_ray = Ray::new(impact, random_hemi_dir.normalize(), ray_ior);

        reflected_ray
    }
    pub(crate) fn refract_ray(&self, impact: Vec3, direction: Vec3, ray_ior: f32) -> (Option<Ray>, f32) {
        let normal = self.normal_at(impact);
        let fresnel = compute_fresnel(direction, normal, ray_ior, self.material.index_of_refraction);

        if self.material.transmission == 0.0 {
            (None, fresnel)
        } else {
            let new_ray_dir = direction.refract(normal, ray_ior / self.material.index_of_refraction);
            (Some(Ray::new(impact, new_ray_dir, self.material.index_of_refraction)), fresnel)
        }
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
        let uv = self.inter_function.uv(at);
        uv.rem_euclid(Vec2::new(1.0, 1.0)) // todo: Put some sort of scale here
    }
}