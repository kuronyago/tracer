use crate::ray::Ray;
use crate::vector::Vector;

pub struct Camera {
    origin: Vector,
    lower_left_corner: Vector,
    horizontal: Vector,
    vertical: Vector,
}

impl Camera {
    pub fn new(
        origin: Vector,
        lower_left_corner: Vector,
        horizontal: Vector,
        vertical: Vector,
    ) -> Self {
        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
