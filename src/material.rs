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
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(Vec3::unit_vector(r.direction()), rec.normal);
        let scattered = Ray(rec.p, reflected + self.fuzz * random_in_unit_sphere());
        if Vec3::dot(scattered.direction(), rec.normal) <= 0.0 {
            return None
        }
        Some((self.albed, scattered))
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
}

impl Material for Dielectric {
    fn scatter(&self, r: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let (outward_normal, ni_over_nt, cosine) = if Vec3::dot(r.direction(), rec.normal) > 0.0 {
            let outward_normal = -rec.normal;
            let ni_over_nt = self.ref_idx;
            let cosine = self.ref_idx + Vec3::dot(r.direction(), rec.normal) / r.direction().length();
            (outward_normal, ni_over_nt, cosine)
        } else {
            let outward_normal = rec.normal;
            let ni_over_nt = 1.0 / self.ref_idx;
            let cosine = - Vec3::dot(r.direction(), rec.normal) / r.direction().length();
            (outward_normal, ni_over_nt, cosine)
        };

        let refracted = refract(r.direction(), outward_normal, ni_over_nt);
        let reflect_prob = if refracted.is_some() {
            schlick(cosine, self.ref_idx)
        } else {
            1.0
        };

        let attenuation = Vec3(1.0,1.0,1.0);
        let mut rng = ::rand::thread_rng(); // TODO
        if rng.gen_range(0.0, 1.0) < reflect_prob {
            let reflected = reflect(r.direction(), rec.normal);
            Some((attenuation, Ray(rec.p, reflected)))
        } else {
            Some((attenuation, Ray(rec.p, refracted.unwrap())))
        }
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

fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0-dt*dt);
    if discriminant > 0.0 {
        Some(ni_over_nt*(uv - n*dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0-ref_idx)/(1.0+ref_idx);
    let r0 = r0*r0;
    r0 + (1.0-r0)*(1.0-cosine).powf(5.0)
}
