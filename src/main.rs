extern crate raytracinginoneweekend;
use std::f32;
use raytracinginoneweekend::*;
use vec3::*;
use ray::*;
use sphere::*;
use hitable_list::*;
use hitable::*;

fn color<T: Hitable>(r: &Ray, world: &T) -> Vec3 {
    if let Some(rec) = world.hit(r, 0.0, f32::MAX) {
        0.5 * (Vec3::unit_vector(rec.normal)+Vec3(1.0,1.0,1.0))
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0-t)*Vec3(1.0,1.0,1.0) + t*Vec3(0.5,0.7,1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let mut world = HitableList::new();
    world.push(Box::new(Sphere{center: Vec3(0.0,0.0,-1.0), radius: 0.5}));
    world.push(Box::new(Sphere{center: Vec3(0.0,-100.5,-1.0), radius: 100.0}));

    let lower_left_corner = Vec3(-2.0,-1.0,-1.0);
    let horizontal = Vec3(4.0,0.0,0.0);
    let vertical = Vec3(0.0,2.0,0.0);
    let origin = Vec3(0.0,0.0,0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(&r, &world);
            let ir = (255.59*col.x()) as u8;
            let ig = (255.59*col.y()) as u8;
            let ib = (255.59*col.z()) as u8;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
