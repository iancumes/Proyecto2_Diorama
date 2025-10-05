use crate::vec3::Vec3;
use crate::texture::{TextureKind, sample as tex_sample};

#[derive(Copy, Clone)]
pub struct Material {
    pub name: &'static str,
    pub texture: TextureKind,
    pub albedo: f32,
    pub specular: f32,
    pub shininess: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub ior: f32,
}

impl Material {
    pub fn color(&self, uv: (f32,f32), p: Vec3) -> Vec3 { tex_sample(self.texture, uv, p) }
}
