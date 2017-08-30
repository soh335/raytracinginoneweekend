extern crate raytracinginoneweekend;
extern crate rand;

use rand::Rng;
use std::f32;

use raytracinginoneweekend::*;
use vec3::*;
use ray::*;
use sphere::*;
use hitable_list::*;
use hitable::*;
use camera::*;
use material::*;

fn color<T: Hitable>(r: &Ray, world: &T, depth: i32) -> Vec3 {
    if let Some((rec, material)) = world.hit(r, 0.000001, f32::MAX) {
        if depth >= 50 {
            return Vec3(0.0, 0.0, 0.0)
        }
        if let Some((attenuation, scatterd)) = material.scatter(r, &rec) {
            return attenuation * color(&scatterd, world, depth+1)
        }
        else {
            return Vec3(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0-t)*Vec3(1.0,1.0,1.0) + t*Vec3(0.5,0.7,1.0)
    }
}

fn random_scene() -> HitableList {
    let mut world = HitableList::new();
    world.push(Box::new(Sphere{center: Vec3(0.0,-1000.0,0.0), radius: 1000.0, material: Box::new(Lambertian{albed: Vec3(0.5,0.5,0.5)})}));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(a as f32 + 0.9*rng.gen_range(0.0,1.0), 0.2, b as f32 + 0.9*rng.gen_range(0.0,1.0));
            if (center - Vec3(4.0,0.2,0.0)).length() > 0.9 {
                let choosen_mut = rng.gen_range(0.0,1.0);
                if choosen_mut < 0.8 {
                    let mut f = || { rng.gen_range(0.0,1.0) * rng.gen_range(0.0,1.0) };
                    world.push(Box::new(Sphere{center: center, radius: 0.2, material: Box::new(Lambertian{albed: Vec3(f(),f(),f())})}));
                } else if choosen_mut < 0.95 {
                    let mut f = || { 0.5 * ( 1.0 + rng.gen_range(0.0,1.0) ) };
                    world.push(Box::new(Sphere{center: center, radius: 0.2, material: Box::new(Metal{fuzz: 1.0, albed: Vec3(f(), f(), f())})}));
                } else {
                    world.push(Box::new(Sphere{center: center, radius: 0.2, material: Box::new(Dielectric{ref_idx: 1.5})}));
                }
            }
        }
    }

    world.push(Box::new(Sphere{center: Vec3(0.0,1.0,0.0), radius: 1.0, material: Box::new(Dielectric{ref_idx: 1.5})}));
    world.push(Box::new(Sphere{center: Vec3(-4.0,1.0,0.0), radius: 1.0, material: Box::new(Lambertian{albed: Vec3(0.4,0.2,0.1)})}));
    world.push(Box::new(Sphere{center: Vec3(4.0,1.0,0.0), radius: 1.0, material: Box::new(Metal{fuzz: 0.0, albed: Vec3(0.7,0.6,0.5)})}));

    world
}

fn main() {
    let nx = 1200;
    let ny = 800;
    //let nx = 200;
    //let ny = 100;
    let ns = 100;

    print!("P3\n{} {}\n255\n", nx, ny);

    let world = random_scene();
    //let mut world = HitableList::new();
    //world.push(Box::new(Sphere{center: Vec3(0.0,0.0,-1.0), radius: 0.5, material: Box::new(Lambertian{albed: Vec3(0.1,0.2,0.5)})}));
    //world.push(Box::new(Sphere{center: Vec3(0.0,-100.5,-1.0), radius: 100.0, material: Box::new(Lambertian{albed: Vec3(0.8,0.8,0.0)})}));
    //world.push(Box::new(Sphere{center: Vec3(1.0,0.0,-1.0), radius: 0.5, material: Box::new(Metal{fuzz: 0.3, albed: Vec3(0.8,0.6,0.2)})}));
    //world.push(Box::new(Sphere{center: Vec3(-1.0,0.0,-1.0), radius: 0.5, material: Box::new(Dielectric{ref_idx: 1.5})}));
    //world.push(Box::new(Sphere{center: Vec3(-1.0,0.0,-1.0), radius: -0.45, material: Box::new(Dielectric{ref_idx: 1.5})}));

    //let look_from = Vec3(3.0,3.0,2.0);
    let look_from = Vec3(13.0,2.0,3.0);
    let look_at   = Vec3(0.0,0.0,-1.0);
    //let dist_to_focus = (look_from - look_at).length();
    let dist_to_focus = 10.0;
    //let aperture = 2.0;
    let aperture = 0.1;
    let camera = Camera::new(look_from, look_at, Vec3(0.0,1.0,0.0), 20.0, nx as f32/ny as f32,
    aperture, dist_to_focus);
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3(0.0,0.0,0.0);
            for _ in 0..ns {
                let u = (i as f32 + rng.gen_range(0.0,1.0)) / nx as f32;
                let v = (j as f32 + rng.gen_range(0.0,1.0)) / ny as f32;
                let r = camera.get_ray(u, v);
                col = col + color(&r, &world,0)
            }
            let col = col / ns as f32;
            let col = Vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());
            let ir = (255.59*col.x()) as u8;
            let ig = (255.59*col.y()) as u8;
            let ib = (255.59*col.z()) as u8;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
