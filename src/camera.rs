use crate::*;

#[derive(Debug, Clone)]
pub struct Camera {
    pub location: Vec3,
    looking_dir: Vec3,
    world_up: Vec3,
    pub hoz_fov: f32,
    pub max_bounces: u32,
    pub samples_per_pixel: u32,
}

// + y is up like minecraft
impl Camera {
    const WORLD_UP: Vec3 = Vec3::Z;
    const FOV: f32 = 75.;
    const SAMPLES_PER_PX: u32 = 10;
    const MAX_BOUNCES: u32 = 10;

    pub(crate) fn right(&self) -> Vec3 {
        self.looking_dir
            .normalize()
            .cross(self.world_up.normalize())
    }

    pub fn up(&self) -> Vec3 {
        self.right().cross(self.looking_dir.normalize())
    }

    pub fn forward(&self) -> Vec3 {
        self.looking_dir
    }

    pub fn new_with_control(location: Vec3, looking_at: Vec3, fov: f32, max_bounces: u32, samples_per_pixel: u32) -> Self {
        let looking_dir = (looking_at - location).normalize();

        Self {
            location,
            looking_dir,
            world_up: Self::WORLD_UP,
            hoz_fov: fov,
            max_bounces,
            samples_per_pixel,
        }
    }

    pub fn new(location: Vec3, looking_at: Vec3) -> Camera {
        let looking_dir = (looking_at - location).normalize();
        Camera {
            location,
            looking_dir,
            world_up: Self::WORLD_UP,
            hoz_fov: Self::FOV,
            max_bounces: Self::MAX_BOUNCES,
            samples_per_pixel: Self::SAMPLES_PER_PX,
        }
    }

    pub fn new_looking_in_dir(location: Vec3, looking_dir: Vec3) -> Camera {
        let looking_dir = looking_dir.normalize();
        Camera {
            location,
            looking_dir,
            world_up: Self::WORLD_UP,
            hoz_fov: Self::FOV,
            max_bounces: Self::MAX_BOUNCES,
            samples_per_pixel: Self::SAMPLES_PER_PX,
        }
    }
}
