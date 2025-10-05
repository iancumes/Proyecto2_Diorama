mod vec3;      use vec3::Vec3;
mod ray;       use ray::Ray;
mod camera;    use camera::Camera;
mod transform;
mod texture;
mod geometry;  use geometry::{Hittable, Hit};
mod skybox;
mod scene;

use raylib::prelude::*;
use raylib::ffi; // UpdateTexture

// ---------------- sombreado/rt ----------------
fn schlick_fresnel(cosine: f32, ior: f32) -> f32 {
    let r0 = ((1.0 - ior) / (1.0 + ior)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

fn trace_ray(r: &Ray, scene: &scene::Scene, depth: u32, atlas: Option<&texture::TextureAtlas>, is_night: bool) -> (Vec3, f32) {
    const EPS: f32 = 1e-3;
    const MAX_DIST: f32 = 1000.0;

    let mut closest: Option<Hit> = None;
    let mut t_max = MAX_DIST;

    for obj in &scene.objects {
        if let Some(h) = obj.hit(r, EPS, t_max) {
            t_max = h.t; closest = Some(h);
        }
    }

    if let Some(hit) = closest {
        let m = &scene.materials[hit.material_id]; // <- borrow, no move
        let base = texture::sample(m.texture, hit.uv, hit.p, atlas);

        // Sombra
        let light_dir = (scene.light_pos - hit.p).normalize();
        let shadow_ray = Ray::new(hit.p + hit.n * EPS*10.0, light_dir);
        let mut in_shadow = false;
        let dist_light = (scene.light_pos - hit.p).length();
        let mut tmax = dist_light - EPS;
        for obj in &scene.objects {
            if let Some(h) = obj.hit(&shadow_ray, EPS, tmax) {
                if h.t < dist_light { in_shadow = true; break; }
                tmax = h.t.min(tmax);
            }
        }

        // Phong con colores de luz
        let mut color = base.hadamard(scene.ambient_color) * scene.ambient;
        if !in_shadow {
            let n = hit.n;
            let l = light_dir;
            let v = (r.origin - hit.p).normalize();
            let h = (l + v).normalize();
            let diff = base.hadamard(scene.light_color) * m.albedo * n.dot(l).max(0.0);
            let spec = scene.light_color * m.specular * n.dot(h).max(0.0).powf(m.shininess);
            color = color + diff + spec;
        }

        if depth == 0 { return (color.clamp01(), hit.t); }

        // Reflexión / Refracción
        let view_dir = (-r.dir).normalize();
        let cosi = view_dir.dot(hit.n).max(0.0);
        let fresnel = schlick_fresnel(cosi, m.ior);

        if m.reflectivity > 0.0 {
            let refl_dir = Vec3::reflect(r.dir, hit.n).normalize();
            let refl_ray = Ray::new(hit.p + hit.n * EPS*10.0, refl_dir);
            let (refl_col, _) = trace_ray(&refl_ray, scene, depth-1, atlas, is_night);
            color = color*(1.0 - m.reflectivity) + refl_col * m.reflectivity;
        }

        if m.transparency > 0.0 {
            let mut n = hit.n;
            let mut eta = 1.0 / m.ior;
            let cosi2 = (-r.dir).dot(n);
            if cosi2 < 0.0 { // dentro
                n = n * -1.0;
                eta = m.ior;
            }
            if let Some(refr_dir) = Vec3::refract(r.dir, n, eta) {
                let refr_ray = Ray::new(hit.p - n * EPS*10.0, refr_dir.normalize());
                let (refr_col, _) = trace_ray(&refr_ray, scene, depth-1, atlas, is_night);
                let kr = fresnel;
                color = color*(1.0 - m.transparency) + (refr_col*(1.0-kr) + color*kr) * m.transparency;
            }
        }

        (color.clamp01(), hit.t)
    } else {
        (skybox::sample_with_mode(r.dir, is_night), MAX_DIST)
    }
}

// Render -> RGBA y Depth
fn render_to_buffers(width: u32, height: u32, cam: &Camera, scene: &scene::Scene, rgba: &mut [u8], depth: &mut [f32], atlas: Option<&texture::TextureAtlas>, is_night: bool) {
    let gamma = 1.0/2.2;
    let mut i = 0usize;

    for y in 0..height {
        for x in 0..width {
            let ray = cam.ray_for_pixel(x, y, width, height);
            let (col, t) = trace_ray(&ray, scene, 5, atlas, is_night);

            let r = (col.x.clamp(0.0,1.0).powf(gamma) * 255.0) as u8;
            let g = (col.y.clamp(0.0,1.0).powf(gamma) * 255.0) as u8;
            let b = (col.z.clamp(0.0,1.0).powf(gamma) * 255.0) as u8;

            rgba[i]   = r;
            rgba[i+1] = g;
            rgba[i+2] = b;
            rgba[i+3] = 255;
            depth[(y*width + x) as usize] = t;
            i += 4;
        }
    }
}

fn depth_to_rgba(depth: &[f32], near: f32, far: f32, rgba: &mut [u8]) {
    let mut i = 0usize;
    for &d in depth {
        let mut z = if d.is_finite() { (d-near)/(far-near) } else { 1.0 };
        z = z.clamp(0.0,1.0);
        let v = ((1.0 - z) * 255.0) as u8;
        rgba[i] = v; rgba[i+1] = v; rgba[i+2] = v; rgba[i+3] = 255;
        i += 4;
    }
}

// Actualiza la textura vía FFI (robusto entre versiones)
fn update_texture_rgba(tex: &mut Texture2D, pixels: &[u8]) {
    unsafe {
        ffi::UpdateTexture(*tex.as_ref(), pixels.as_ptr() as *const std::ffi::c_void);
    }
}

fn main() {
    let width: i32 = std::env::var("W").ok().and_then(|s| s.parse().ok()).unwrap_or(800);
    let height: i32 = std::env::var("H").ok().and_then(|s| s.parse().ok()).unwrap_or(450);

    let (mut rl, thread) = raylib::init()
        .size(width, height)
        .title("Diorama Raytracer (Rust + raylib)")
        .build();

    rl.set_target_fps(30);

    // --- atlas de texturas desde archivos ---
    std::fs::create_dir_all("assets").ok();
    let atlas_paths = [
        "assets/grass.png", // id 0
        "assets/dirt.png",  // id 1
        "assets/stone.png", // id 2
        "assets/metal2.png", // id 3
        "assets/glass.png", // id 4
        "assets/water.png", // id 5
    ];
    let atlas = texture::TextureAtlas::load_from_paths(&atlas_paths);

    // Buffers
    let mut pixels = vec![0u8; (width*height*4) as usize];
    let mut depthbuf = vec![f32::INFINITY; (width*height) as usize];

    // Texture destino
    let blank = Image::gen_image_color(width, height, Color::BLACK);
    let mut tex = rl.load_texture_from_image(&thread, &blank).unwrap();

    // Estado
    let mut yaw: f32 = 35f32.to_radians();
    let mut cam_dist: f32 = 8.5;
    let mut fov: f32 = 60.0;
    let mut auto_rotate = true;
    let mut show_depth = false;
    let mut is_night = true;  // Empezar en modo nocturno para ver la luna

    while !rl.window_should_close() {
        // input
        if rl.is_key_down(KeyboardKey::KEY_A) { yaw -= 0.02; }
        if rl.is_key_down(KeyboardKey::KEY_D) { yaw += 0.02; }
        if rl.is_key_down(KeyboardKey::KEY_W) { cam_dist -= 0.05; }
        if rl.is_key_down(KeyboardKey::KEY_S) { cam_dist += 0.05; }
        cam_dist -= rl.get_mouse_wheel_move() * 0.2;
        if rl.is_key_pressed(KeyboardKey::KEY_R) { auto_rotate = !auto_rotate; }
        if rl.is_key_pressed(KeyboardKey::KEY_Z) { show_depth = !show_depth; }
        if rl.is_key_pressed(KeyboardKey::KEY_N) { is_night = !is_night; }  // Toggle día/noche
        if rl.is_key_pressed(KeyboardKey::KEY_Q) { fov = (fov-1.0).clamp(25.0, 90.0); }
        if rl.is_key_pressed(KeyboardKey::KEY_E) { fov = (fov+1.0).clamp(25.0, 90.0); }
        if rl.is_key_pressed(KeyboardKey::KEY_P) { rl.take_screenshot(&thread, "out/frame.png"); } // <- con thread

        if auto_rotate { yaw += 0.01; }

        // escena + cámara
        let sc = scene::Scene::diorama_with_mode(yaw, is_night);
        let aspect = width as f32 / height as f32;
        let cam_pos = Vec3::new(0.0, 2.5, cam_dist);
        let cam = Camera { pos: cam_pos, target: Vec3::new(0.0,0.5,0.0), up: Vec3::new(0.0,1.0,0.0), fov_deg: fov, aspect };

        // render
        render_to_buffers(width as u32, height as u32, &cam, &sc, &mut pixels, &mut depthbuf, Some(&atlas), is_night);
        if show_depth {
            let mut tmp = vec![0u8; pixels.len()];
            depth_to_rgba(&depthbuf, 0.1, 50.0, &mut tmp);
            update_texture_rgba(&mut tex, &tmp);
        } else {
            update_texture_rgba(&mut tex, &pixels);
        }

        // draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&tex, 0, 0, Color::WHITE);
        d.draw_text(
            &format!("A/D rotar | Wheel/W/S zoom | R auto:{} | Z depth:{} | N {}| Q/E FOV:{:.0} | P screenshot",
                     if auto_rotate {"ON"} else {"OFF"},
                     if show_depth {"ON"} else {"OFF"},
                     if is_night {"NOCHE"} else {"DÍA"},
                     fov),
            10, 10, 18, Color::WHITE
        );
    }
}
