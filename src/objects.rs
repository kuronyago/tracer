use crate::vector::Vector;

pub struct HitRecord {
    pub t: f64,
    pub v: Vector,
    pub normal: Vector,
}
