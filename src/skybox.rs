use crate::vec3::Vec3;

// Hash simple para estrellas
fn hash(n: u32) -> f32 {
    let mut x = n; 
    x ^= x << 13; 
    x ^= x >> 17; 
    x ^= x << 5;
    (x as f32) / (u32::MAX as f32)
}

pub fn sample(dir: Vec3) -> Vec3 {
    sample_with_mode(dir, false)
}

pub fn sample_with_mode(dir: Vec3, is_night: bool) -> Vec3 {
    if is_night {
        // MODO NOCTURNO
        let t = (dir.y*0.5 + 0.5).clamp(0.0,1.0);
        let top = Vec3::new(0.02, 0.03, 0.08);     // azul oscuro profundo
        let bottom = Vec3::new(0.08, 0.10, 0.15);  // gris azulado en horizonte
        let mut base = bottom*(1.0 - t) + top*t;

        // Luna (más grande y brillante)
        let moon_dir = Vec3::new(-0.4, 0.7, 0.3).normalize();
        let dot_moon = dir.normalize().dot(moon_dir).max(0.0);
        let moon_glow = dot_moon.powf(150.0) * 2.5 + dot_moon.powf(800.0) * 5.0;
        base = base + Vec3::new(0.9, 0.95, 1.0) * moon_glow;

        // Estrellas (solo en la parte superior del cielo)
        if dir.y > 0.1 {
            let scale = 100.0;
            let ix = (dir.x * scale) as i32;
            let iy = (dir.y * scale) as i32;
            let iz = (dir.z * scale) as i32;
            let star_id = (ix * 73856093 ^ iy * 19349663 ^ iz * 83492791) as u32;
            let star_val = hash(star_id);
            if star_val > 0.998 { // solo algunas direcciones tienen estrellas
                let brightness = hash(star_id.wrapping_mul(7919)) * 0.6 + 0.4;
                base = base + Vec3::new(1.0, 1.0, 0.95) * brightness;
            }
        }

        base.clamp01()
    } else {
        // MODO DÍA (original)
        let t = (dir.y*0.5 + 0.5).clamp(0.0,1.0);
        let top = Vec3::new(0.25, 0.55, 0.95);
        let bottom = Vec3::new(0.85, 0.93, 1.0);
        let base = bottom*(1.0 - t) + top*t;

        let sun_dir = Vec3::new(0.3, 0.6, -0.7).normalize();
        let dot = dir.normalize().dot(sun_dir).max(0.0);
        let glow = dot.powf(100.0) * 3.0 + dot.powf(1000.0) * 7.0;
        (base + Vec3::new(1.0,0.9,0.7)*glow).clamp01()
    }
}
