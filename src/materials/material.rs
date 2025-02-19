use std::fmt::Debug;
use image::Rgb32FImage;
use crate::{Ray, Vec3, Vec3Colour};
use palette::named::{BLACK, WHITE};
use crate::utils::ColourChange;

pub trait RenderMaterial: Debug + Sync {
    fn scatter_ray(&self, ray: Ray, impact: Vec3) -> Option<Ray>;
    fn colour_ray(&self, ray: Ray, impact: Vec3) -> Option<Vec3Colour>;

}

// enum Texture {
//     Solid(Vec3),
//     Image(Rgb32FImage)
// }

// pub struct RenderMaterial {
//     pub base_colour: Vec3,         // "Diffuse" or "albedo" color
//     pub emission_colour: Vec3,     // Emissive color (light emission)
//     pub index_of_refraction: f32,  // For refraction calculations (Snell's law)
//     pub transmission: f32,         // 0 = opaque, 1 = fully transmissive
//     pub roughness: f32,            // 0 = perfect mirror, 1 = very rough
//     pub metallic: f32,             // 0 = dielectric, 1 = fully metallic
// }
//
// impl RenderMaterial {
//     pub(crate) fn new(base_colour: Vec3, emission_colour: Vec3, index_of_refraction: f32, transmission: f32, roughness: f32, metallic: f32) -> RenderMaterial {
//         Self {
//             base_colour,
//             emission_colour,
//             index_of_refraction,
//             transmission,
//             roughness,
//             metallic,
//         }
//     }
//
//     pub fn light_source(colour: Vec3Colour) -> Self {
//         Self {
//             base_colour: colour,
//             emission_colour: colour,
//             index_of_refraction: 0.0,
//             transmission: 0.0,
//             roughness: 0.0,
//             metallic: 0.0,
//         }
//     }
//
//     /// Perfect vacuum or air approximation
//     pub const AIR: Self = Self {
//         base_colour: Vec3::new(1.0, 1.0, 1.0),
//         emission_colour: Vec3::new(0.0, 0.0, 0.0),
//         index_of_refraction: 1.0,
//         transmission: 1.0,
//         roughness: 0.0,
//         metallic: 0.0,
//     };
//
//     /// Water approximation
//     pub const WATER: Self = Self {
//         base_colour: Vec3::new(0.0, 0.05, 0.2),
//         emission_colour: Vec3::new(0.0, 0.0, 0.0),
//         index_of_refraction: 1.333,
//         transmission: 1.0,
//         roughness: 0.0,
//         metallic: 0.0,
//     };
//
//     /// Glass approximation
//     pub const GLASS: Self = Self {
//         base_colour: Vec3::new(1.0, 1.0, 1.0),
//         emission_colour: Vec3::new(0.0, 0.0, 0.0),
//         index_of_refraction: 1.5,
//         transmission: 1.0,
//         roughness: 0.0,
//         metallic: 0.0,
//     };
//
//     /// Idealised mirror (perfectly reflective metal)
//     pub const MIRROR: Self = Self {
//         base_colour: Vec3::new(1.0, 1.0, 1.0),
//         emission_colour: Vec3::new(0.0, 0.0, 0.0),
//         index_of_refraction: 0.0, // Not used here
//         transmission: 0.0,
//         roughness: 0.0,
//         metallic: 1.0,
//     };
//
//     /// Approximate gold color
//     pub const GOLD: Self = Self {
//         base_colour: Vec3::new(1.0, 0.84, 0.0),
//         emission_colour: Vec3::new(0.0, 0.0, 0.0),
//         index_of_refraction: 0.0, // Not used here
//         transmission: 0.0,
//         roughness: 0.0,
//         metallic: 1.0,
//     };
//
//     /// A simple emissive material (blue glow)
//     pub const EMISSIVE_BLUE: Self = Self {
//         base_colour: Vec3::new(0.0, 0.0, 0.0),
//         emission_colour: Vec3::new(0.0, 0.0, 1.0),
//         index_of_refraction: 0.0, // Not used here
//         transmission: 0.0,
//         roughness: 0.0,
//         metallic: 0.0,
//     };
//
//     /// A generic plastic with moderate roughness
//     pub fn plastic(base_colour: Vec3) -> Self {
//         Self {
//             base_colour,
//             emission_colour: Vec3::new(0.0, 0.0, 0.0),
//             index_of_refraction: 0.0, // Not used here since transmission = 0.0
//             transmission: 0.0,
//             roughness: 1.0,
//             metallic: 0.0,
//         }
//     }
//
//     pub fn super_matte(base_colour: Vec3) -> Self {
//         Self {
//             base_colour,
//             emission_colour: Vec3::new(0.0, 0.0, 0.0),
//             index_of_refraction: 0.0, // Not used here since transmission = 0.0
//             transmission: 0.0,
//             roughness: 5.0,
//             metallic: 0.0,
//         }
//     }
//
// }
