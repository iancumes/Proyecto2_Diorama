use crate::vec3::Vec3;
use raylib::prelude::*;

#[derive(Copy, Clone)]
pub enum TextureKind {
    Image { id: usize },                       // imagen externa del atlas
    Checker { a: Vec3, b: Vec3, scale: f32 }, // procedurales (fallback)
    Grass, Dirt, Stone, Metal, GlassTint, Water,
}

pub struct CpuImage {
    pub w: i32,
    pub h: i32,
    pub rgba: Vec<u8>, // RGBA8
}
impl CpuImage {
    #[inline]
    pub fn sample_nearest(&self, u: f32, v: f32) -> Vec3 {
        let mut uu = u.fract(); if uu < 0.0 { uu += 1.0 }
        let mut vv = v.fract(); if vv < 0.0 { vv += 1.0 }
        let x = (uu * (self.w as f32 - 1.0)).round() as i32;
        let y = (vv * (self.h as f32 - 1.0)).round() as i32;
        let i = ((y*self.w + x) * 4) as usize;
        let r = self.rgba[i] as f32 / 255.0;
        let g = self.rgba[i+1] as f32 / 255.0;
        let b = self.rgba[i+2] as f32 / 255.0;
        Vec3::new(r,g,b)
    }
}

pub struct TextureAtlas {
    pub images: Vec<CpuImage>,
}

impl TextureAtlas {
    pub fn load_from_paths(paths: &[&str]) -> Self {
        let mut images = Vec::new();
        for p in paths {
            match Image::load_image(p) {
                Ok(img) => {
                    let w = img.width();
                    let h = img.height();
                    // Nota: get_image_data() devuelve ImageColors; usamos .iter()
                    let data = img.get_image_data();
                    let mut rgba = Vec::with_capacity((w*h*4) as usize);
                    for c in data.iter() {
                        rgba.push(c.r); rgba.push(c.g); rgba.push(c.b); rgba.push(c.a);
                    }
                    images.push(CpuImage { w, h, rgba });
                    eprintln!("Cargada textura '{}': {}x{}", p, w, h);
                }
                Err(_) => {
                    eprintln!("(aviso) No se pudo cargar '{}'. Usaré fallback procedural.", p);
                }
            }
        }
        Self { images }
    }

    #[inline]
    pub fn get(&self, id: usize) -> Option<&CpuImage> { self.images.get(id) }
}

// ---------------- Procedurales (fallback) ----------------

fn hash(n: i32) -> f32 {
    let mut x = n as u32; x ^= x << 13; x ^= x >> 17; x ^= x << 5;
    (x as f32) / (u32::MAX as f32)
}
fn noise2(ix: i32, iy: i32) -> f32 {
    let h = hash(ix * 73856093 ^ iy * 19349663); h * 2.0 - 1.0
}

pub fn sample(kind: TextureKind, uv: (f32, f32), p: Vec3, atlas: Option<&TextureAtlas>) -> Vec3 {
    let (u, v) = uv;
    match kind {
        TextureKind::Image { id } => {
            if let Some(a) = atlas {
                if let Some(img) = a.get(id) { return img.sample_nearest(u, v).clamp01(); }
            }
            // Fallback si no hay imagen: magenta
            return Vec3::new(1.0, 0.0, 1.0);
        }
        TextureKind::Checker{a,b,scale} => {
            let s = ((u*scale).floor() as i32 + (v*scale).floor() as i32) & 1;
            if s==0 { a } else { b }
        }
        TextureKind::Grass => {
            let n = noise2((u*64.0) as i32, (v*64.0) as i32) * 0.08;
            Vec3::new(0.24, 0.58, 0.18) + Vec3::new(n, n*0.5, n)
        }
        TextureKind::Dirt => {
            let s = ((u*16.0).floor() as i32 ^ (v*16.0).floor() as i32) & 1;
            if s==0 { Vec3::new(0.40,0.26,0.12) } else { Vec3::new(0.34,0.22,0.11) }
        }
        TextureKind::Stone => {
            let n = noise2((p.x*5.0) as i32, (p.z*5.0) as i32)*0.15;
            Vec3::new(0.55,0.55,0.58) + Vec3::new(n,n,n)
        }
        TextureKind::Metal => {
            let b = ((u*64.0).sin()*0.04).abs();
            Vec3::new(0.75,0.78,0.82) + Vec3::new(b,b,b)
        }
        TextureKind::GlassTint => Vec3::new(0.6,0.8,1.0)*0.7,
        TextureKind::Water => {
            let w = ((u*30.0 + (p.z+p.x)*0.2).sin() + (v*30.0 + p.x*0.2).cos())*0.02;
            Vec3::new(0.2, 0.6, 0.8) + Vec3::new(0.0,0.05,0.08) * w
        }
    }.clamp01()
}
