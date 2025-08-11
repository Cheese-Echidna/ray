use crate::hit::Hit;
use crate::materials::material::RenderMaterial;
use crate::utils::{bounce_across_normal, build_orthonormal_basis, random_point_on_unit_sphere, ColourChange};
use crate::{Ray, Vec3Colour};
use glam::{Mat3, UVec2, Vec2, Vec3};
use image::Rgb32FImage;

#[derive(Debug)]
pub struct Texture {
    texture: ImageHolder,
    normals: Option<ImageHolder>,
    normal_strength: f32,
    scale: Vec2,
    roughness: f32,
    // rotation: f32
}

impl Texture {
    pub fn new(
        image_path: impl AsRef<std::path::Path>,
        normals_path: Option<impl AsRef<std::path::Path>>,
        scale: Vec2,
        roughness: f32,
    ) -> Self {
        Self {
            texture: ImageHolder::new(image_path),
            normals: normals_path.map(|x| ImageHolder::new(x)),
            normal_strength: 10.0,
            scale,
            roughness,
        }
    }

    fn sample_image(&self, uv: Vec2) -> Vec3Colour {
        self.texture.sample(uv, self.scale)
    }

    fn sample_normals(&self, uv: Vec2) -> Option<Vec3Colour> {
        self.normals.as_ref().map(|x| x.sample(uv, self.scale).map(|x| x * 2. - 1.))
    }
    fn get_sampled_normal(&self, hit: Hit) -> Vec3 {
        if let Some(sample_normal) = self.sample_normals(hit.uv) {
            let (t, b, n) = (hit.uv_derivatives.0.normalize(), hit.uv_derivatives.1.normalize(), hit.normal);
            let tbn = Mat3::from_cols(t, b, n);

            let perturbed_normal = (tbn.transpose() * sample_normal).normalize();
            // let final_normal = (hit.normal + self.normal_strength * perturbed_normal).normalize();

            perturbed_normal
        } else {
            hit.normal
        }
    }
}

impl RenderMaterial for Texture {
    fn scatter_ray(&self, hit: Hit) -> Option<Ray> {
        let normal = self.get_sampled_normal(hit);
        let new_dir = bounce_across_normal(hit.ray.direction(), normal);
        let dir = new_dir + random_point_on_unit_sphere() * self.roughness;

        Some(Ray::new(hit.impact, dir))
    }

    fn colour(&self, hit: Hit, future_colour: Vec3Colour) -> Vec3Colour {
        self.sample_image(hit.uv) * future_colour
    }
}

#[derive(Debug)]
struct ImageHolder {
    image: Rgb32FImage,
    size: Vec2,
}

impl ImageHolder {
    fn new(path: impl AsRef<std::path::Path>) -> Self {
        let image = image::open(path).unwrap().to_rgb32f();
        let size = UVec2::new(image.width(), image.height()).as_vec2();
        Self { image, size }
    }
    fn sample(&self, uv: Vec2, scale: Vec2) -> Vec3Colour {
        let uv = (uv * self.size * scale) % self.size;
        let uv = uv.as_uvec2();
        let pixel = self.image.get_pixel(uv.x, uv.y);
        pixel.to_vec3()
    }
}
