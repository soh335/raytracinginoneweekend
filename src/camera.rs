use vec3::Vec3;
use ray::Ray;
use std::f32;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, vup: Vec3, vfov: f32, aspect: f32) -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = half_height * aspect;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);
        let origin = look_from;
        let lower_left_corner = origin - half_width*u - half_height*v - w;
        let horizontal = 2.0*half_width*u;
        let vertical = 2.0*half_height*v;
        Camera{
            lower_left_corner: lower_left_corner,
            horizontal:        horizontal,
            vertical:          vertical,
            origin:            origin,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
