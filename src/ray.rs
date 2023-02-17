use glam::Vec3A;

pub struct Ray {
    pub origin: Vec3A,
    pub direction: Vec3A,
}

impl Ray {
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Ray {
            origin: origin,
            direction: direction.normalize(),
        }
    }

    pub fn origin(&self) -> Vec3A {
        self.origin
    }
    pub fn direction(&self) -> Vec3A {
        self.direction
    }
    pub fn point_at_parameter(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}
