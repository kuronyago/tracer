mod camera;
mod objects;
mod ray;
mod vector;

use camera::Camera;
use objects::Object::{Multiple, Single};
use objects::Sphere;
use rand::prelude::*;
use vector::Vector;

fn color(r: &ray::Ray, world: &objects::Object) -> Vector {
    let hit = world.hit(r, 0.0, std::f64::MAX);

    match hit {
        Some(record) => {
            let target: Vector = record.p + record.normal + random_in_unit_sphere();
            let r_a: ray::Ray = ray::Ray::new(record.p, target - record.p);
            return 0.5 * color(&r_a, world);
        }
        None => {
            let unit = r.direction.unit();
            let t = (unit.b + 1.0) * 0.5;
            let res = (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0);
            return res;
        }
    }
}

fn random_in_unit_sphere() -> Vector {
    let mut rng = thread_rng();

    let unit = Vector::new(1.0, 1.0, 1.0);

    loop {
        let p: Vector =
            2.0 * Vector::new(rng.gen::<f64>(), rng.gen::<f64>(), rng.gen::<f64>()) - unit;

        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = vector::Vector::new(-2.0, -1.0, -1.0);
    let horizontal = vector::Vector::new(4.0, 0.0, 0.0);
    let vertical = vector::Vector::new(0.0, 2.0, 0.0);
    let origin = vector::Vector::new(0.0, 0.0, 0.0);

    let objects: Vec<objects::Object> = vec![
        Single(Sphere::new(vector::Vector::new(0.0, 0.0, -1.0), 0.5)),
        Single(Sphere::new(vector::Vector::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let world = Multiple(objects);

    let cam = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = cam.get_ray(u, v);

            let col = color(&r, &world);

            let ir = (255.99 * col.a) as i32;
            let ig = (255.99 * col.b) as i32;
            let ib = (255.99 * col.c) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
