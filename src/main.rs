mod objects;
mod ray;
mod vector;

fn hit_sphere(center: vector::Vector, radius: f64, ray: &ray::Ray) -> f64 {
    let oc = ray.origin - center;

    let a: f64 = vector::Vector::dot(&ray.direction, &ray.direction);
    let b: f64 = 2.0 * vector::Vector::dot(&oc, &ray.direction);
    let c: f64 = vector::Vector::dot(&oc, &oc) - radius * radius;
    let discriminant: f64 = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-1.0 * b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: &ray::Ray) -> vector::Vector {
    let tt: f64 = hit_sphere(vector::Vector::new(0.0, 0.0, -1.0), 0.5, r);

    if tt > 0.0 {
        let u = r.point_at_parameter(tt) - vector::Vector::new(0.0, 0.0, -1.0);

        let n = vector::Vector::unit(&u);
        return 0.5 * vector::Vector::new(n.a + 1.0, n.b + 1.0, n.c + 1.0);
    }

    let unit_direction = vector::Vector::unit(&r.direction);
    let t = 0.5 * (unit_direction.b + 1.0);

    (1.0 - t) * vector::Vector::new(1.0, 1.0, 1.0) + t * vector::Vector::new(0.5, 0.7, 1.0)
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = vector::Vector::new(-2.0, -1.0, -1.0);
    let horizontal = vector::Vector::new(4.0, 0.0, 0.0);
    let vertical = vector::Vector::new(0.0, 2.0, 0.0);
    let origin = vector::Vector::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = (i as f64) / (nx as f64);
            let v = (j as f64) / (ny as f64);

            let r = ray::Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let col = color(&r);

            let ir = (255.99 * col.a) as i32;
            let ig = (255.99 * col.b) as i32;
            let ib = (255.99 * col.c) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
