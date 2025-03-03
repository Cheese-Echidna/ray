use glam::{UVec2, Vec2, Vec3};
use image::Rgb32FImage;
use crate::hit::Hit;
use crate::materials::material::RenderMaterial;
use crate::{Ray, Vec3Colour};
use crate::utils::ColourChange;

#[derive(Debug)]
pub struct Texture {
    texture: ImageHolder,
    normals: Option<ImageHolder>,
    scale: Vec2,
    // rotation: f32
}

impl Texture {
    pub fn new(image_path: impl AsRef<std::path::Path>, normals_path: Option<impl AsRef<std::path::Path>>, scale: Vec2) -> Self {
        Self {
            texture: ImageHolder::new(image_path),
            normals: normals_path.map(|x| ImageHolder::new(x)),
            scale,
        }
    }

    fn sample_image(&self, uv: Vec2) -> Vec3Colour {
        self.texture.sample(uv, self.scale)
    }

    fn sample_normals(&self, uv: Vec2) -> Option<Vec3Colour> {
        self.normals.as_ref().map(|x| x.sample(uv, self.scale))
    }
}

impl RenderMaterial for Texture {
    fn scatter_ray(&self, hit: Hit) -> Option<Ray> {
        // hit.normal
        None
    }

    fn colour(&self, hit: Hit, future_colour: Vec3Colour) -> Vec3Colour {
        self.sample_image(hit.uv)
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