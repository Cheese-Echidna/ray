use crate::{Ray, Vec3};
use glam::Vec2;
use palette::LinSrgb;
use rand::random;
use std::f32::consts::PI;

macro_rules! dprintln {
    ($($arg:tt)*) => {if ::std::cfg!(debug_assertions) {::std::println!($($arg)*);}};
}

pub(crate) use dprintln;

pub fn vec_format(v: Vec3) -> String {
    format!("({:.4}, {:.4}, {:.4})", v.x, v.y, v.z)
}

pub fn bounce_across_normal(incoming: Vec3, normal: Vec3) -> Vec3 {
    let incoming = incoming.normalize();
    let normal = normal.normalize();
    (incoming - 2.0 * incoming.dot(normal) * normal).normalize()
}

pub fn random_cosine_direction(normal: Vec3) -> Vec3 {
    let r1: f32 = random(); // in [0, 1)
    let r2: f32 = random(); // in [0, 1)

    let phi = 2.0 * PI * r1;
    // r2 is in [0,1], so sqrt(r2) is in [0,1]
    // We want z = sqrt(1 - r2) to reflect the cos weighting.
    let z = (1.0 - r2).sqrt();

    // radius in the "x-y plane" for that z
    let sin_theta = r2.sqrt();

    let x = phi.cos() * sin_theta;
    let y = phi.sin() * sin_theta;

    let hemi = Vec3::new(x, y, z);

    let (a, b, c) = build_orthonormal_basis(normal);
    local_to_world(hemi, a, b, c)
}

/// Vector component of u in the direction of v
pub(crate) fn vector_projection(u: Vec3, v: Vec3) -> Vec3 {
    u.dot(v) / v.length_squared() * v
}

/// Vector component of u perpendicular to the direction of v
pub(crate) fn perpendicular_projection(u: Vec3, v: Vec3) -> Vec3 {
    u - vector_projection(u, v)
}

/// Scalar projection of u in the direction of v
pub(crate) fn scalar_projection(u: Vec3, v: Vec3) -> f32 {
    u.dot(v) / v.length_squared()
}

pub(crate) fn build_orthonormal_basis(n: Vec3) -> (Vec3, Vec3, Vec3) {
    // Ensure normal is normalized
    let w = n.normalize();

    // Pick a helper vector 'a' that is not parallel to 'w'
    let a = if w.x.abs() > 0.9 {
        Vec3::new(0.0, 1.0, 0.0)
    } else {
        Vec3::new(1.0, 0.0, 0.0)
    };

    // v = w x a
    let v = w.cross(a).normalize();
    // u = v x w
    let u = v.cross(w).normalize();

    (u, v, w)
}

/// Ray going from A to B
pub fn compute_fresnel(ray_dir: Vec3, normal: Vec3, ior1: f32, ior2: f32) -> f32 {
    let cos_theta = -ray_dir.dot(normal).max(0.0);
    let f0 = ((ior1 - ior2) / (ior1 + ior2)).powi(2);
    fresnel_schlick(cos_theta, f0)
}

fn fresnel_schlick(cos_theta: f32, f0: f32) -> f32 {
    f0 + (1.0 - f0) * (1.0 - cos_theta).powf(5.0)
}

fn local_to_world(local: Vec3, u: Vec3, v: Vec3, w: Vec3) -> Vec3 {
    let [x, y, z] = local.to_array();
    x * (u) + y * (v) + z * (w)
}

pub trait ColourChange {
    fn to_vec3(self) -> Vec3;
    fn from_vec3(x: Vec3) -> Self;
}

impl ColourChange for LinSrgb<f32> {
    fn to_vec3(self) -> Vec3 {
        Vec3::new(self.red, self.green, self.blue)
    }

    fn from_vec3(x: Vec3) -> Self {
        Self::new(x.x, x.y, x.z)
    }
}

impl ColourChange for palette::rgb::Rgb<palette::encoding::Srgb, u8> {
    fn to_vec3(self) -> Vec3 {
        let x = <palette::rgb::Rgb<palette::encoding::Srgb, u8> as Into<LinSrgb<f32>>>::into(self);
        x.to_vec3()
    }

    fn from_vec3(x: Vec3) -> Self {
        let x = LinSrgb::from_vec3(x);
        let y: palette::Srgb<f32> = x.into_encoding();
        let z: palette::Srgb<u8> = y.into_format();
        z
    }
}

pub fn random_point_on_unit_sphere() -> Vec3 {
    let u: f32 = random();
    let v: f32 = random();
    let theta = 2. * PI * u;
    let phi = (2.0 * v - 1.).acos();

    let dir = Vec3::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos()).normalize();
    dir
}
