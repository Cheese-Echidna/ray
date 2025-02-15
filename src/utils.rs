macro_rules! dprintln {
    ($($arg:tt)*) => {if ::std::cfg!(debug_assertions) {::std::println!($($arg)*);}};
}

macro_rules! convert {
    ($value:expr) => {
        Box::new($value) as Box<dyn RenderObject>
    };
}

pub(crate) use convert;
pub(crate) use dprintln;

use rand::random;
use std::f64::consts::PI;

use crate::Vec3;

pub fn vec_format(v: Vec3) -> String {
    format!("({:.4}, {:.4}, {:.4})", v.x, v.y, v.z)
}

pub fn bounce_across_normal(incoming: Vec3, normal: Vec3) -> Vec3 {
    let incoming = incoming.normalize();
    let normal = normal.normalize();
    (incoming - 2.0 * incoming.dot(normal) * normal).normalize()
}

pub fn random_cosine_direction(normal: Vec3) -> Vec3 {
    let r1: f64 = random(); // in [0, 1)
    let r2: f64 = random(); // in [0, 1)

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

fn local_to_world(local: Vec3, u: Vec3, v: Vec3, w: Vec3) -> Vec3 {
    let [x, y, z] = local.to_array();
    x * (u) + y * (v) + z * (w)
}

pub(crate) fn fix_normal(direction: Vec3, normal: Vec3) -> Vec3 {
    normal
        * if normal.dot(direction) < 0.0 {
            -1.0
        } else {
            1.0
        }
}
