use vec3::Vec3;
use ray::Ray;
use std::f32;
use rand::Rng;

fn random_in_unit_disk() -> Vec3 {
    let mut rng = ::rand::thread_rng(); // TODO
    loop {
        let p = 2.0 * Vec3(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0,1.0), 0.0) - Vec3(1.0,1.0,0.0);
        if Vec3::dot(p,p) < 1.0 {
            return p
        }
    }
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Self {
        let lens_radius = aperture / 2.0;
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = half_height * aspect;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let origin = look_from;
        let lower_left_corner = origin - half_width*u*focus_dist - half_height*v*focus_dist -
        w*focus_dist;
        let horizontal = 2.0*half_width*u*focus_dist;
        let vertical = 2.0*half_height*v*focus_dist;
        Camera{
            lower_left_corner: lower_left_corner,
            horizontal:        horizontal,
            vertical:          vertical,
            origin:            origin,
            lens_radius:       lens_radius,
            u:                 u,
            v:                 v,
            w:                 w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray(self.origin + offset, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
    }
}
