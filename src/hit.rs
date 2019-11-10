use crate::ray::Ray;
use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Record {
    pub t: f64,
    pub p: Vector,
    pub normal: Vector,
}

pub trait Hitable {
    fn hit(&self, r: &Ray, min: f64, max: f64) -> Option<Record>;
}
