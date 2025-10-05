use crate::vec3::Vec3;
use crate::texture::TextureKind;
use crate::geometry::{Object, Cube};

#[derive(Clone)]
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

pub struct Scene {
    pub objects: Vec<Object>,
    pub materials: Vec<Material>,
    pub light_pos: Vec3,
    pub light_color: Vec3,
    pub ambient: f32,
    pub ambient_color: Vec3,
}

impl Scene {
    pub fn diorama(angle_y: f32) -> Self {
        Self::diorama_with_mode(angle_y, false)
    }
    
    pub fn diorama_with_mode(angle_y: f32, is_night: bool) -> Self {
        // IDs esperados en el atlas:
        // 0: grass, 1: dirt, 2: stone, 3: metal, 4: glass, 5: water, 6: lamp (vidrio luminoso)
        let grass = Material { name:"Grass", texture:TextureKind::Image{ id:0 }, albedo:0.85, specular:0.15, shininess:32.0, transparency:0.0, reflectivity:0.0, ior:1.0 };
        let dirt  = Material { name:"Dirt",  texture:TextureKind::Image{ id:1 }, albedo:0.9,  specular:0.05, shininess:8.0,  transparency:0.0, reflectivity:0.0, ior:1.0 };
        let stone = Material { name:"Stone", texture:TextureKind::Image{ id:2 }, albedo:0.8,  specular:0.1,  shininess:16.0, transparency:0.0, reflectivity:0.0, ior:1.0 };
        let metal = Material { name:"Metal", texture:TextureKind::Image{ id:3 }, albedo:0.1, specular:0.9, shininess:128.0, transparency:0.0, reflectivity:0.8, ior:1.0 };
        let glass = Material { name:"Glass", texture:TextureKind::Image{ id:4 }, albedo:0.05, specular:0.9, shininess:128.0, transparency:0.85, reflectivity:0.1, ior:1.52 };
        let water = Material { name:"Water", texture:TextureKind::Image{ id:5 }, albedo:0.2, specular:0.5, shininess:64.0, transparency:0.7, reflectivity:0.15, ior:1.33 };
        let lamp  = Material { name:"Lamp",  texture:TextureKind::Image{ id:6 }, albedo:0.3, specular:0.8, shininess:96.0, transparency:0.35, reflectivity:0.2, ior:1.52 };

        let mats = vec![grass, dirt, stone, metal, glass, water, lamp];

        let mut objs: Vec<Object> = vec![];
        
        // ====== SUELO BASE (terreno con grass y dirt) ======
        for z in -4..=4 {
            for x in -4..=4 {
                let r = (x*x + z*z) as f32;
                let mat = if r < 12.0 { 0 } else { 1 }; // grass dentro, dirt fuera
                let center = Vec3::new(x as f32, -0.75, z as f32);
                objs.push(Object::Cube(Cube::new(center, Vec3::new(0.5,0.25,0.5), 0.0, mat)));
            }
        }
        
        // ====== CASTILLO CENTRAL (piedra) ======
        // Torre principal (centro)
        for i in 0..5 {
            objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.5 + i as f32 * 0.4, 0.0), Vec3::new(0.35, 0.4, 0.35), 0.0, 2)));
        }
        // Corona de la torre (metal)
        objs.push(Object::Cube(Cube::new(Vec3::new(0.0, 1.5, 0.0), Vec3::new(0.25, 0.15, 0.25), 0.0, 3)));
        
        // Murallas (4 muros de piedra)
        for i in 0..3 {
            objs.push(Object::Cube(Cube::new(Vec3::new(-1.5, -0.3 + i as f32 * 0.35, 0.0), Vec3::new(0.15, 0.35, 1.0), 0.0, 2)));
            objs.push(Object::Cube(Cube::new(Vec3::new(1.5, -0.3 + i as f32 * 0.35, 0.0), Vec3::new(0.15, 0.35, 1.0), 0.0, 2)));
        }
        for i in 0..2 {
            objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.3 + i as f32 * 0.35, -1.5), Vec3::new(0.8, 0.35, 0.15), 0.0, 2)));
            objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.3 + i as f32 * 0.35, 1.5), Vec3::new(0.8, 0.35, 0.15), 0.0, 2)));
        }
        
        // Torres esquineras (4 torres pequeñas)
        let corners = [
            Vec3::new(-1.5, 0.0, -1.5),
            Vec3::new(1.5, 0.0, -1.5),
            Vec3::new(-1.5, 0.0, 1.5),
            Vec3::new(1.5, 0.0, 1.5),
        ];
        for &pos in &corners {
            for i in 0..3 {
                objs.push(Object::Cube(Cube::new(pos + Vec3::new(0.0, i as f32 * 0.35, 0.0), Vec3::new(0.25, 0.35, 0.25), 0.0, 2)));
            }
            // Techo metálico
            objs.push(Object::Cube(Cube::new(pos + Vec3::new(0.0, 1.05, 0.0), Vec3::new(0.2, 0.1, 0.2), 0.0, 3)));
        }
        
        // ====== EDIFICIOS LATERALES ======
        // Edificio izquierdo (piedra con ventanas de vidrio)
        for i in 0..4 {
            objs.push(Object::Cube(Cube::new(Vec3::new(-3.0, -0.3 + i as f32 * 0.4, 1.0), Vec3::new(0.4, 0.4, 0.6), 0.0, 2)));
        }
        // Ventanas de vidrio
        objs.push(Object::Cube(Cube::new(Vec3::new(-2.6, 0.3, 1.0), Vec3::new(0.08, 0.25, 0.4), 0.0, 4)));
        objs.push(Object::Cube(Cube::new(Vec3::new(-2.6, 0.9, 1.0), Vec3::new(0.08, 0.25, 0.4), 0.0, 4)));
        
        // Edificio derecho (piedra con decoración metálica)
        for i in 0..3 {
            objs.push(Object::Cube(Cube::new(Vec3::new(3.0, -0.3 + i as f32 * 0.4, -1.0), Vec3::new(0.5, 0.4, 0.5), 0.0, 2)));
        }
        // Decoración metálica en techo
        objs.push(Object::Cube(Cube::new(Vec3::new(3.0, 0.9, -1.0), Vec3::new(0.3, 0.1, 0.3), 0.0, 3)));
        
        // ====== PUENTE Y FUENTE DE AGUA ======
        // Fuente de agua (centro-sur)
        objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.4, 2.5), Vec3::new(0.6, 0.15, 0.6), 0.0, 2))); // borde
        objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.35, 2.5), Vec3::new(0.5, 0.12, 0.5), 0.0, 5))); // agua
        // Pilar central de la fuente
        objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.1, 2.5), Vec3::new(0.1, 0.2, 0.1), 0.0, 2)));
        objs.push(Object::Cube(Cube::new(Vec3::new(0.0, 0.1, 2.5), Vec3::new(0.15, 0.05, 0.15), 0.0, 3)));
        
        // Puente de piedra (norte)
        for i in 0..4 {
            objs.push(Object::Cube(Cube::new(Vec3::new(-1.0 + i as f32 * 0.5, -0.45, -2.8), Vec3::new(0.25, 0.08, 0.3), 0.0, 2)));
        }
        // Barandas metálicas del puente
        for i in 0..5 {
            objs.push(Object::Cube(Cube::new(Vec3::new(-1.0 + i as f32 * 0.5, -0.25, -2.6), Vec3::new(0.04, 0.15, 0.04), 0.0, 3)));
        }
        
        // ====== DECORACIONES ======
        // Esferas metálicas decorativas
        objs.push(Object::Cube(Cube::new(Vec3::new(-2.5, -0.35, -2.0), Vec3::new(0.18, 0.18, 0.18), 0.0, 3)));
        objs.push(Object::Cube(Cube::new(Vec3::new(2.5, -0.35, 2.0), Vec3::new(0.18, 0.18, 0.18), 0.0, 3)));
        
        // Cristales decorativos (pequeños) - usa material lamp (ID 6) para que se vean mejor
        objs.push(Object::Cube(Cube::new(Vec3::new(-1.0, -0.3, -0.8), Vec3::new(0.12, 0.2, 0.12), 0.3, 6)));
        objs.push(Object::Cube(Cube::new(Vec3::new(1.0, -0.3, 0.8), Vec3::new(0.12, 0.2, 0.12), -0.3, 6)));
        
        // Pilares ornamentales (esquinas exteriores)
        let pillars = [
            Vec3::new(-3.5, 0.0, -3.0),
            Vec3::new(3.5, 0.0, -3.0),
            Vec3::new(-3.5, 0.0, 3.0),
            Vec3::new(3.5, 0.0, 3.0),
        ];
        for &pos in &pillars {
            objs.push(Object::Cube(Cube::new(pos + Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.15, 0.3, 0.15), 0.0, 2)));
            objs.push(Object::Cube(Cube::new(pos + Vec3::new(0.0, -0.2, 0.0), Vec3::new(0.15, 0.3, 0.15), 0.0, 2)));
            objs.push(Object::Cube(Cube::new(pos + Vec3::new(0.0, 0.1, 0.0), Vec3::new(0.12, 0.12, 0.12), 0.0, 3)));
        }
        
        // Camino de piedra (entrada)
        for i in 0..6 {
            objs.push(Object::Cube(Cube::new(Vec3::new(0.0, -0.48, -3.5 + i as f32 * 0.5), Vec3::new(0.4, 0.05, 0.2), 0.0, 2)));
        }
        
        // Jardines pequeños (grass elevado)
        objs.push(Object::Cube(Cube::new(Vec3::new(-2.0, -0.45, 0.5), Vec3::new(0.3, 0.08, 0.3), 0.0, 0)));
        objs.push(Object::Cube(Cube::new(Vec3::new(2.0, -0.45, -0.5), Vec3::new(0.3, 0.08, 0.3), 0.0, 0)));
        
        // Estanque pequeño adicional
        objs.push(Object::Cube(Cube::new(Vec3::new(-3.0, -0.45, -1.5), Vec3::new(0.4, 0.08, 0.4), 0.0, 5)));

        // ====== ANTORCHAS/LÁMPARAS (para iluminación nocturna) ======
        // Antorchas en las torres esquineras
        let torch_positions = [
            Vec3::new(-1.5, 1.2, -1.5),
            Vec3::new(1.5, 1.2, -1.5),
            Vec3::new(-1.5, 1.2, 1.5),
            Vec3::new(1.5, 1.2, 1.5),
        ];
        for &pos in &torch_positions {
            // Poste de metal
            objs.push(Object::Cube(Cube::new(pos + Vec3::new(0.0, -0.15, 0.0), Vec3::new(0.05, 0.15, 0.05), 0.0, 3)));
            // Lámpara de vidrio (emite luz) - usa material lamp (ID 6)
            objs.push(Object::Cube(Cube::new(pos, Vec3::new(0.1, 0.1, 0.1), 0.0, 6)));
        }
        
        // Lámparas en la torre principal
        for i in 0..4 {
            let angle = i as f32 * std::f32::consts::PI / 2.0;
            let x = angle.cos() * 0.5;
            let z = angle.sin() * 0.5;
            objs.push(Object::Cube(Cube::new(Vec3::new(x, 1.3, z), Vec3::new(0.08, 0.08, 0.08), 0.0, 6)));
        }
        
        // Faroles en el camino
        objs.push(Object::Cube(Cube::new(Vec3::new(-0.6, -0.2, -2.5), Vec3::new(0.06, 0.06, 0.06), 0.0, 6)));
        objs.push(Object::Cube(Cube::new(Vec3::new(0.6, -0.2, -2.5), Vec3::new(0.06, 0.06, 0.06), 0.0, 6)));

        // Rotación global (match sin warning)
        let rot = crate::transform::Mat3::rotate_y(angle_y);
        for o in objs.iter_mut() {
            match o {
                crate::geometry::Object::Cube(c) => {
                    let p = c.tr.pos;
                    let rotated = rot.mul_vec3(p);
                    c.tr = crate::transform::Transform::new(rot, rotated);
                }
            }
        }

        // Configuración de iluminación según el modo
        let (light_pos, light_color, ambient, ambient_color) = if is_night {
            // MODO NOCTURNO: Luz lunar azulada desde arriba
            (
                Vec3::new(-8.0, 12.0, 5.0),           // Luna alta y lejana
                Vec3::new(0.6, 0.7, 1.0),             // Luz azulada fría
                0.08,                                  // Ambient muy bajo
                Vec3::new(0.15, 0.20, 0.35)           // Ambient azulado oscuro
            )
        } else {
            // MODO DÍA: Sol cálido
            (
                Vec3::new(5.0, 6.0, -3.0),            // Sol
                Vec3::new(1.0, 0.95, 0.85),           // Luz cálida amarillenta
                0.15,                                  // Ambient normal
                Vec3::new(1.0, 1.0, 1.0)              // Ambient neutro
            )
        };

        Self {
            objects: objs,
            materials: mats,
            light_pos,
            light_color,
            ambient,
            ambient_color,
        }
    }
}
