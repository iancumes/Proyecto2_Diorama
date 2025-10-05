use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    pub pos: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov_deg: f32,
    pub aspect: f32,
}

impl Camera {
    pub fn ray_for_pixel(&self, x: u32, y: u32, w: u32, h: u32) -> Ray {
        // Base on look-at vectors
        let wv = (self.pos - self.target).normalize();
        let u = self.up.cross(wv).normalize();
        let v = wv.cross(u);
        let fov = (self.fov_deg.to_radians() * 0.5).tan();

        // NDC -> dirección
        let ndc_x = (2.0 * (x as f32 + 0.5) / w as f32 - 1.0) * self.aspect * fov;
        let ndc_y = (1.0 - 2.0 * (y as f32 + 0.5) / h as f32) * fov;
        let dir = (u * ndc_x + v * ndc_y + (wv * -1.0)).normalize();
        Ray::new(self.pos, dir)
    }
}
