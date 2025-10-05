#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };
    pub const ONE:  Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn add(self, o: Vec3) -> Self { Self::new(self.x+o.x, self.y+o.y, self.z+o.z) }
    pub fn sub(self, o: Vec3) -> Self { Self::new(self.x-o.x, self.y-o.y, self.z-o.z) }
    pub fn mul(self, k: f32) -> Self { Self::new(self.x*k, self.y*k, self.z*k) }
    pub fn hadamard(self, o: Vec3) -> Self { Self::new(self.x*o.x, self.y*o.y, self.z*o.z) }
    pub fn dot(self, o: Vec3) -> f32 { self.x*o.x + self.y*o.y + self.z*o.z }
    pub fn cross(self, o: Vec3) -> Self {
        Self::new(self.y*o.z - self.z*o.y, self.z*o.x - self.x*o.z, self.x*o.y - self.y*o.x)
    }
    pub fn length(self) -> f32 { self.dot(self).sqrt() }
    pub fn normalize(self) -> Self {
        let l = self.length(); if l>0.0 { self.mul(1.0/l) } else { self }
    }
    pub fn clamp01(self) -> Self {
        fn c(x: f32) -> f32 { x.max(0.0).min(1.0) }
        Self::new(c(self.x), c(self.y), c(self.z))
    }
    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 { v.sub(n.mul(2.0 * v.dot(n))) }
    pub fn refract(v: Vec3, n: Vec3, eta: f32) -> Option<Vec3> {
        let cosi = (-v.dot(n)).max(-1.0).min(1.0);
        let k = 1.0 - eta*eta*(1.0 - cosi*cosi);
        if k < 0.0 { None } else { Some(v.mul(eta).add(n.mul(eta*cosi - k.sqrt()))) }
    }
}

use std::ops::{Add, Sub, Mul, Neg};
impl Add for Vec3 { type Output=Self; fn add(self,o:Self)->Self{ Self::new(self.x+o.x,self.y+o.y,self.z+o.z) } }
impl Sub for Vec3 { type Output=Self; fn sub(self,o:Self)->Self{ Self::new(self.x-o.x,self.y-o.y,self.z-o.z) } }
impl Mul<f32> for Vec3 { type Output=Self; fn mul(self,k:f32)->Self{ Self::new(self.x*k,self.y*k,self.z*k) } }
impl Mul<Vec3> for Vec3 { type Output=Self; fn mul(self,o:Vec3)->Self{ Self::new(self.x*o.x,self.y*o.y,self.z*o.z) } }
impl Neg for Vec3 { type Output=Self; fn neg(self)->Self{ Self::new(-self.x, -self.y, -self.z) } }
