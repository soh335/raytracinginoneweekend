use vec3::Vec3;
use ray::Ray;
use hitable::*;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    fn build_hit_record(&self, t: f32, r: &Ray) -> HitRecord {
        let p = r.point_at_parameter(t);
        let normal = (p - self.center) / self.radius;
        HitRecord{t, p, normal}
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(r.direction(), r.direction());
        let b = 2.0 * Vec3::dot(oc, r.direction());
        let c = Vec3::dot(oc, oc) - self.radius*self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant > 0.0 {
            let discriminant_sqrt = discriminant.sqrt();
            let temp = (-b - discriminant_sqrt ) / 2.0 * a;
            if temp < t_max && temp > t_min {
                return Some(self.build_hit_record(temp, r))
            }
            let temp = (-b + discriminant_sqrt) / 2.0 * a;
            if temp < t_max && temp > t_min {
                return Some(self.build_hit_record(temp, r))
            }
        }
        return None
    }
}
