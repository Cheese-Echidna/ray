use crate::*;
use rand::random;
use std::f32::consts::PI;
use std::fmt::Debug;
use palette::encoding::Linear;
use palette::rgb::Rgb;
use crate::materials::material::RenderMaterial;
use crate::utils::{bounce_across_normal, compute_fresnel, random_cosine_direction};
use crate::intersections::intersection::RenderIntersection;

#[derive(Debug)]
pub struct RenderObject {
    inter_function: Box<dyn RenderIntersection>,
    material: Box<dyn RenderMaterial>
}

impl RenderObject {
    pub fn new(b: Box<dyn RenderIntersection>, m: Box<dyn RenderMaterial>) -> Self {
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
    pub(crate) fn hit_on_inside(&self, impact: Vec3, direction: Vec3) -> bool {
        let normal = self.normal_at(impact);
        normal.dot(direction) < 0.0
    }
    pub(crate) fn scatter_ray(&self, impact: Vec3, direction: Vec3, ray_ior: f32) -> Ray {
        let normal = utils::fix_normal(direction, self.normal_at(impact));
        let reflect_dir = bounce_across_normal(direction, normal);

        let roughness = self.material.roughness * 0.1;

        let random_hemi_dir = (1. - roughness) * reflect_dir + roughness * random_cosine_direction(normal);

        let reflected_ray = Ray::new(impact, random_hemi_dir.normalize(), ray_ior);

        reflected_ray
    }
    pub(crate) fn refract_ray(&self, impact: Vec3, direction: Vec3, ray_ior: f32) -> (Option<Ray>, f32) {
        let normal = utils::fix_normal(direction, self.normal_at(impact));

        let roughness = self.material.roughness * 0.1;

        let new_index = if self.hit_on_inside(impact, direction) {
            1.0
        } else {
            self.material.transmission
        };

        let fresnel = compute_fresnel(direction, normal, ray_ior, new_index);

        if self.material.transmission == 0.0 {
            (None, fresnel)
        } else {
            let new_ray_dir = direction.refract(normal, ray_ior / new_index);
            let random_hemi_dir = (1. - roughness) * new_ray_dir + roughness * random_cosine_direction(normal);

            (Some(Ray::new(impact, random_hemi_dir, new_index)), fresnel)
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