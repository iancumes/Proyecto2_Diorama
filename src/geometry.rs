use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::transform::Transform;

pub struct Hit {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
    pub uv: (f32,f32),
    pub material_id: usize,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<Hit>;
}

pub struct Cube {
    pub half: Vec3,
    pub tr: Transform,
    pub material_id: usize,
}

impl Cube {
    pub fn new(center: Vec3, half: Vec3, rot_y_rad: f32, material_id: usize) -> Self {
        let rot = crate::transform::Mat3::rotate_y(rot_y_rad);
        let tr = Transform::new(rot, center);
        Self { half, tr, material_id }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        // AABB en espacio local del cubo
        let ro = self.tr.world_to_local_point(r.origin);
        let rd = self.tr.world_to_local_dir(r.dir);

        let min = self.half * -1.0;
        let max = self.half;

        fn slab(o: f32, d: f32, mn: f32, mx: f32) -> (f32,f32) {
            let inv = 1.0 / d;
            let mut t0 = (mn - o) * inv;
            let mut t1 = (mx - o) * inv;
            if t0 > t1 { std::mem::swap(&mut t0, &mut t1); }
            (t0, t1)
        }

        let (mut t0x, mut t1x) = slab(ro.x, rd.x, min.x, max.x);
        let (t0y, t1y) = slab(ro.y, rd.y, min.y, max.y);
        let (t0z, t1z) = slab(ro.z, rd.z, min.z, max.z);
        t0x = t0x.max(t0y).max(t0z);
        t1x = t1x.min(t1y).min(t1z);

        if t1x < t0x || t1x < tmin || t0x > tmax { return None; }
        let t_hit = if t0x >= tmin { t0x } else { t1x };
        if t_hit < tmin || t_hit > tmax { return None; }

        let p_local = ro + rd * t_hit;
        let eps = 1e-4;
        let n_local = if (p_local.x - max.x).abs() < eps { Vec3::new(1.0,0.0,0.0) }
            else if (p_local.x - min.x).abs() < eps { Vec3::new(-1.0,0.0,0.0) }
            else if (p_local.y - max.y).abs() < eps { Vec3::new(0.0,1.0,0.0) }
            else if (p_local.y - min.y).abs() < eps { Vec3::new(0.0,-1.0,0.0) }
            else if (p_local.z - max.z).abs() < eps { Vec3::new(0.0,0.0,1.0) }
            else { Vec3::new(0.0,0.0,-1.0) };

        let (u,v) = if n_local.x.abs() > 0.5 {
            ((p_local.z - min.z)/(max.z-min.z), (p_local.y - min.y)/(max.y-min.y))
        } else if n_local.y.abs() > 0.5 {
            ((p_local.x - min.x)/(max.x-min.x), (p_local.z - min.z)/(max.z-min.z))
        } else {
            ((p_local.x - min.x)/(max.x-min.x), (p_local.y - min.y)/(max.y-min.y))
        };

        let p_world = r.at(t_hit);
        let n_world = self.tr.local_to_world_dir(n_local).normalize();

        Some(Hit { t: t_hit, p: p_world, n: n_world, uv: (u,v), material_id: self.material_id })
    }
}

pub enum Object { Cube(Cube) }

impl Hittable for Object {
    fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> Option<Hit> {
        match self { Object::Cube(c) => c.hit(r,tmin,tmax) }
    }
}
