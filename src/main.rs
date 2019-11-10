mod hit;
mod objects;
mod ray;
mod sphere;
mod vector;

use hit::Hitable;
use objects::World;
use sphere::Sphere;

fn color(r: &ray::Ray, world: &World<Hitable>) -> vector::Vector {
    let hit = world.hit(r, 0.0, std::f64::MAX);

    match hit {
        Some(record) => {
            let res = 0.5
                * vector::Vector::new(
                    record.normal.a + 1.0,
                    record.normal.b + 1.0,
                    record.normal.c + 1.0,
                );
            return res;
        }
        None => {
            let unit = r.direction.unit();
            let t = (unit.b + 1.0) * 0.5;
            let res = (1.0 - t) * vector::Vector::new(1.0, 1.0, 1.0)
                + t * vector::Vector::new(0.5, 0.7, 1.0);
            return res;
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

    let objects: Vec<World<T: Hitable>> = vec![
        World::Single(Sphere::new(vector::Vector::new(0.0, 0.0, -1.0), 0.5)),
        World::Single(Sphere::new(vector::Vector::new(0.0, -100.5, -1.0), 100.0)),
    ];

    let world = World::Multiple(objects);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(&r, &world);

            let ir = (255.99 * col.a) as i32;
            let ig = (255.99 * col.b) as i32;
            let ib = (255.99 * col.c) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
