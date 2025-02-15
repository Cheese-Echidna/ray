use crate::*;
use glam::DVec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub(crate) start: Vec3,
    direction: Vec3,
}

impl Ray {
    pub(crate) fn direction(&self) -> DVec3 {
        self.direction
    }
    pub(crate) fn start(&self) -> Vec3 {
        self.start
    }
}

impl Ray {
    pub fn new(start: Vec3, direction: Vec3) -> Ray {
        Ray {
            start,
            direction: direction.normalize(),
        }
    }

    pub fn new_from_to(from: Vec3, to: Vec3) -> Self {
        Self::new(from, to - from)
    }

    pub fn pos_at_length(&self, l: Length) -> Vec3 {
        self.start + self.direction * l
    }
}
