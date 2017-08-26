use ray::Ray;
use hitable::HitRecord;
use vec3::Vec3;
use rand::Rng;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albed: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        //let target = rec.p + Vec3::unit_vector(rec.normal) + random_in_unit_sphere();
        Some((self.albed, Ray(rec.p, target - rec.p)))
    }
}

pub struct Metal {
    pub albed: Vec3,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(Vec3::unit_vector(r.direction()), rec.normal);
        let scattered = Ray(rec.p, reflected);
        if Vec3::dot(scattered.direction(), rec.normal) <= 0.0 {
            return None
        }
        Some((self.albed, scattered))
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = ::rand::thread_rng(); // TODO
    loop {
        let p = Vec3(rng.gen_range(-1.0,1.0),rng.gen_range(-1.0,1.0),rng.gen_range(-1.0,1.0));
        if p.squard_length() < 1.0 {
            return p
        }
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * Vec3::dot(v,n)*n
}
