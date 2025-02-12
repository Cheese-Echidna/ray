use glam::DVec2;
use crate::*;
use palette::FromColor;

#[derive(Debug, Clone)]
pub struct Camera {
    location: Vec3,
    looking_dir: Vec3,
    world_up: Vec3,
    hoz_fov: f64,
}

// + y is up like minecraft
impl Camera {
    const WORLD_UP: Vec3 = Vec3::Z;
    fn right(&self) -> Vec3 {
        self.looking_dir.normalize().cross(self.world_up.normalize())
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(self.looking_dir.normalize())
    }

    pub fn forward(&self) -> Vec3 {
        self.looking_dir
    }

    pub fn new(location: Vec3, looking_at: Vec3) -> Camera {
        let looking_dir = (looking_at - location).normalize();
        Camera {
            location,
            looking_dir,
            world_up: Self::WORLD_UP,
            hoz_fov: 75.,
        }
    }

    pub fn new_looking_in_dir(location: Vec3, looking_dir: Vec3) -> Camera {
        let looking_dir = looking_dir.normalize();
        Camera {
            location,
            looking_dir,
            world_up: Self::WORLD_UP,
            hoz_fov: 75.,
        }
    }


    pub fn trace(&self, scene: &Scene, image_prop: Vec2) -> Option<Srgb> {
        let ray = self.get_outgoing_ray(image_prop);

        let global_illumination = 0.;

        let light_intersection = ray.trace_light_sources(scene, 0.0);
        let object_intersection = ray.trace(&scene, 0.0);

        object_intersection.map(|(object, intersection)| {
            let intensity = global_illumination
                + scene
                    .lights
                    .iter()
                    .map(|x| x.intensity_for(ray, intersection, object, scene))
                    .sum::<f64>();
            let mut colour: palette::Hsl = object.colour().into_color();
            colour.lightness = intensity as f32;
            colour.into_color()
        }).or(light_intersection.map(|x| x.0.colour()))
    }

    fn get_outgoing_ray(&self, image_prop: DVec2) -> Ray {
        // x and y are camera coords
        // both range from -0.5 to 0.5
        let [x, y] = image_prop.to_array();

        let aspect_ratio = 1.0; // Assuming a square image
        let tan_fov = (self.hoz_fov.to_radians() / 2.0).tan();

        // Calculate horizontal and vertical offsets
        let right_offset = self.right() * (x * aspect_ratio * tan_fov);
        let up_offset = self.up() * (y * tan_fov);

        // Calculate the direction vector
        let direction = (self.forward() + right_offset + up_offset).normalize();

        let ray = Ray::new(self.location, direction);
        ray
    }
}
