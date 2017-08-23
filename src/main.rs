extern crate raytracinginoneweekend;
use raytracinginoneweekend::*;
use vec3::*;
use ray::*;

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant > 0.0
}

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(Vec3(0.0,0.0,-1.0),0.5,r) {
        return Vec3(1.0,0.0,0.0)
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0-t)*Vec3(1.0,1.0,1.0) + t*Vec3(0.5,0.7,1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let lower_left_corner = Vec3(-2.0,-1.0,-1.0);
    let horizontal = Vec3(4.0,0.0,0.0);
    let vertical = Vec3(0.0,2.0,0.0);
    let origin = Vec3(0.0,0.0,0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r);
            let ir = (255.59*col.x()) as i64;
            let ig = (255.59*col.y()) as i64;
            let ib = (255.59*col.z()) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
