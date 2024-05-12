use bevy::math::Vec3;

pub trait Vec3Extensions {
    fn add_y(&self, value: f32) -> Self;
    fn set_y(&self, value: f32) -> Self;
}

impl Vec3Extensions for Vec3 {
    fn add_y(&self, value: f32) -> Self { Vec3::new(self.x, self.y + value, self.z) }
    fn set_y(&self, value: f32) -> Self { Vec3::new(self.x, value, self.z) }
}
