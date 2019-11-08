use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
}

pub struct Sphere {
    center: Vector,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64) -> Self {
        Sphere { center, radius }
    }

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a: f64 = Vector::dot(&ray.direction, &ray.direction);
        let b: f64 = Vector::dot(&oc, &ray.direction);
        let c: f64 = Vector::dot(&oc, &oc) - self.radius * self.radius;

        let disciminant: f64 = b * b - a * c;

        if disciminant > 0.0 {
            {
                let temp = (-b - (b * b - a * c).sqrt()) / a;

                if temp < t_max && temp > t_min {
                    let p = ray.point_at_parameter(temp);
                    let normal = (p - self.center) / self.radius;
                    let record: HitRecord = HitRecord {
                        t: temp,
                        p: p,
                        normal: normal,
                    };

                    return Some(record);
                }
            }

            {
                let temp = (-b + (b * b - a * c).sqrt()) / a;
                if temp < t_max && temp > t_min {
                    let p = ray.point_at_parameter(temp);
                    let normal = (p - self.center) / self.radius;
                    let record: HitRecord = HitRecord {
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

pub enum Object {
    Single(Sphere),
    Multiple(Vec<Object>),
}

impl Object {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Single(s) => s.hit(ray, t_min, t_max),
            Object::Multiple(objects) => hit_mult(&objects, ray, t_min, t_max),
        }
    }
}

fn hit_mult(objects: &Vec<Object>, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut closest: Option<HitRecord> = None;

    for object in objects.iter() {
        if let Some(h) = object.hit(ray, t_min, t_max) {
            match closest {
                None => closest = Some(h),
                Some(res) => {
                    if h.t < res.t {
                        closest = Some(h)
                    }
                }
            }
        }
    }

    closest
}
