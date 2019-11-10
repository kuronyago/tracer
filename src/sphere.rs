use crate::hit::{Hitable, Record};
use crate::ray::Ray;
use crate::vector::Vector;

pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, min: f64, max: f64) -> Option<Record> {
        let oc = r.origin - self.center;
        let a: f64 = Vector::dot(&r.direction, &r.direction);
        let b: f64 = Vector::dot(&oc, &r.direction);
        let c: f64 = Vector::dot(&oc, &oc) - self.radius * self.radius;

        let disciminant: f64 = b * b - a * c;

        if disciminant > 0.0 {
            {
                let temp = (-b - (b * b - a * c).sqrt()) / a;

                if temp < max && temp > min {
                    let p = r.point_at_parameter(temp);
                    let normal = (p - self.center) / self.radius;
                    let record = Record {
                        t: temp,
                        p: p,
                        normal: normal,
                    };

                    return Some(record);
                }
            }

            {
                let temp = (-b + (b * b - a * c).sqrt()) / a;
                if temp < max && temp > min {
                    let p = r.point_at_parameter(temp);
                    let normal = (p - self.center) / self.radius;
                    let record = Record {
                        t: temp,
                        p: p,
                        normal: normal,
                    };

                    return Some(record);
                }
            }
        }

        None
    }
}
