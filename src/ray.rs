use crate::vec3::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray { pub origin: Vec3, pub dir: Vec3 }

impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self { Self { origin, dir: dir.normalize() } }
    pub fn at(&self, t: f32) -> Vec3 { self.origin + self.dir * t }
}
