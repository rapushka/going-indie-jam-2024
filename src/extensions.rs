use bevy::math::{Vec2, Vec3};

pub trait Vec3Extensions {
    fn add_y(&self, value: f32) -> Self;
    fn set_y(&self, value: f32) -> Self;
}

impl Vec3Extensions for Vec3 {
    fn add_y(&self, value: f32) -> Self { Vec3::new(self.x, self.y + value, self.z) }
    fn set_y(&self, value: f32) -> Self { Vec3::new(self.x, value, self.z) }
}

pub trait Vec2Extensions {
    fn as_flat(&self) -> Vec3;
}

impl Vec2Extensions for Vec2 {
    fn as_flat(&self) -> Vec3 { Vec3::new(self.x, 0.0, self.y) }
}
