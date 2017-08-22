extern crate raytracinginoneweekend;
use raytracinginoneweekend::*;
use vec3::*;

fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3(i as f64 /nx as f64, j as f64 /ny as f64, 0.2);
            let ir = (255.59*col.x()) as i64;
            let ig = (255.59*col.y()) as i64;
            let ib = (255.59*col.z()) as i64;
            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
