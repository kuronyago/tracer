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
            0.5 * color(&ray::Ray::new(record.p, target - record.p), world)
        }
        None => {
            let unit = r.direction.unit();
            let t = (unit.b + 1.0) * 0.5;
            (1.0 - t) * Vector::new(1.0, 1.0, 1.0) + t * Vector::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_in_unit_sphere() -> Vector {
    let mut rng: ThreadRng = rand::thread_rng();
    let mut p: Vector = Vector::new(std::f64::MAX, std::f64::MAX, std::f64::MAX);
    loop {
        if p.squared_length() < 1.0 {
            break;
        }

        p = Vector::new(
            rng.gen_range(-1.5, 1.5),
            rng.gen_range(-1.5, 1.5),
            rng.gen_range(-1.5, 1.5),
        );
    }
    p
}

fn main() {
    let nx = 200;
    let ny = 100;
    let nz = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = vector::Vector::new(-2.0, -1.0, -1.0);
    let horizontal = vector::Vector::new(4.0, 0.0, 0.0);
    let vertical = vector::Vector::new(0.0, 2.0, 0.0);
    let origin = vector::Vector::new(0.0, 0.0, 0.0);

    let objects: Vec<objects::Object> = vec![
        Single(Sphere::new(vector::Vector::new(0.0, 0.0, -1.0), 0.5)),
        Single(Sphere::new(vector::Vector::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let mut rng = rand::thread_rng();

    let world = Multiple(objects);

    let camera = Camera::new(origin, lower_left_corner, horizontal, vertical);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col: Vector = Vector::new(0.0, 0.0, 0.0);

            for _ in 0..nz {
                let u = (f64::from(i) + rng.gen_range(0.0, 1.0)) / f64::from(nx);
                let v = (f64::from(j) + rng.gen_range(0.0, 1.0)) / f64::from(ny);
                let r = camera.get_ray(u, v);
                col = col + color(&r, &world);
            }

            col = col / f64::from(nz);

            col = Vector::new(col.a.sqrt(), col.b.sqrt(), col.c.sqrt());

            let ir = (255.99 * col.a) as i32;
            let ig = (255.99 * col.b) as i32;
            let ib = (255.99 * col.c) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
