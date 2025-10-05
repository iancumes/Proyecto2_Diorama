use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Mat3 { pub c0: Vec3, pub c1: Vec3, pub c2: Vec3 }

impl Mat3 {
    pub fn rotate_y(theta: f32) -> Self {
        let (s, c) = theta.sin_cos();
        Self {
            c0: Vec3::new(c,0.0,-s),
            c1: Vec3::new(0.0,1.0,0.0),
            c2: Vec3::new(s,0.0,c),
        }
    }
    pub fn mul_vec3(&self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.c0.x*v.x + self.c1.x*v.y + self.c2.x*v.z,
            self.c0.y*v.x + self.c1.y*v.y + self.c2.y*v.z,
            self.c0.z*v.x + self.c1.z*v.y + self.c2.z*v.z,
        )
    }
    pub fn transpose(&self) -> Self {
        Self {
            c0: Vec3::new(self.c0.x, self.c1.x, self.c2.x),
            c1: Vec3::new(self.c0.y, self.c1.y, self.c2.y),
            c2: Vec3::new(self.c0.z, self.c1.z, self.c2.z),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Transform {
    pub rot: Mat3,
    pub inv_rot: Mat3,
    pub pos: Vec3,
}

impl Transform {
    pub fn new(rot: Mat3, pos: Vec3) -> Self { Self { inv_rot: rot.transpose(), rot, pos } }
    pub fn world_to_local_point(&self, p: Vec3) -> Vec3 { self.inv_rot.mul_vec3(p - self.pos) }
    pub fn world_to_local_dir(&self, d: Vec3) -> Vec3 { self.inv_rot.mul_vec3(d) }
    pub fn local_to_world_point(&self, p: Vec3) -> Vec3 { self.pos + self.rot.mul_vec3(p) }
    pub fn local_to_world_dir(&self, d: Vec3) -> Vec3 { self.rot.mul_vec3(d) }
}
