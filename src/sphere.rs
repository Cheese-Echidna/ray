use crate::*;
use crate::utils::{bounce_across_normal, random_cosine_direction};

#[derive(Debug, Clone)]
pub struct Sphere {
    pub centre: Vec3,
    pub radius: Length,
    pub colour: LinSrgb,
    pub emissivity: f32,
    pub roughness: f64,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: Length, colour: LinSrgb, emissivity: f32, roughness: f64) -> Sphere {
        Sphere {centre, radius, colour, emissivity, roughness }
    }

    fn private_intersects(&self, ray: Ray) -> Vec<Length> {
        // Offset - the position of the sphere relative to the start of the ray
        let os = ray.start() - &self.centre;

        // Calculate a,b,c so we can plug them into the quadratic formula. Except
        // a should be the squared Euclidean distance of the ray direction,
        // but ray directions are normalised to a unit vector, so a will be 1,
        // so we can ignore it.
        let b = 2.0 * os.dot(ray.direction());
        let c = os.length_squared() - self.radius.powi(2);

        let discriminant = b.powi(2) - 4.0 * c;

        let intersections = if discriminant < 0.0 {
            vec![]
        } else if discriminant.abs() <= 0.0001 {
            vec![-b / 2.0]
        } else {
            let root = discriminant.sqrt();
            vec![(-b - root) / 2.0, (-b + root) / 2.0]
        };
        intersections.into_iter().filter(|x| *x >= 0.0).collect()
    }

    fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.centre).normalize()
    }
}

impl RenderObject for Sphere {
    fn intersects(&self, ray: Ray) -> Vec<Vec3> {
        self.private_intersects(ray).into_iter().map(|x| ray.pos_at_length(x)).collect()
    }

    fn scatter(&self, impact: Vec3, direction: Vec3) -> Option<(LinSrgb, Ray)> {
        let normal = self.normal_at(impact);
        let reflect_dir = bounce_across_normal(direction, normal);

        // let random_hemi_dir = reflect_dir + self.roughness * random_cosine_direction(normal);

        Some((self.colour, Ray::new(impact, reflect_dir.normalize())))
    }

    fn emission(&self, impact: Vec3, direction: Vec3) -> LinSrgb {
        let closeness = (-direction).normalize().dot(self.normal_at(impact)).abs() as f32;
        // this could cause issues if hitting inside of sphere ^^^
        // OPTIONS: (x*0.5 + 0.5) or x.abs() or x.clamp(0.0, 1.0)
        self.colour * self.emissivity * closeness
    }
}