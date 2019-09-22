use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
    pub a: f64,
    pub b: f64,
    pub c: f64,
}

impl Vector {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Vector { a, b, c }
    }

    pub fn squared_length(&self) -> f64 {
        self.a * self.a + self.b * self.b + self.c * self.c
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn unit(&self) -> Vector {
        *self / self.length()
    }

    pub fn dot(l: &Vector, r: &Vector) -> f64 {
        l.a * r.a + l.b * r.b + l.c * r.c
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, x: f64) -> Vector {
        Vector {
            a: self.a / x,
            b: self.b / x,
            c: self.c / x,
        }
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, v: Vector) -> Vector {
        Vector {
            a: self.a - v.a,
            b: self.b - v.b,
            c: self.c - v.c,
        }
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, v: Vector) -> Vector {
        Vector {
            a: self.a + v.a,
            b: self.b + v.b,
            c: self.c + v.c,
        }
    }
}

impl ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, x: Vector) -> Vector {
        Vector {
            a: self.a * x.a,
            b: self.b * x.b,
            c: self.c * x.c,
        }
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, x: f64) -> Vector {
        Vector {
            a: self.a * x,
            b: self.b * x,
            c: self.c * x,
        }
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, x: Vector) -> Vector {
        Vector {
            a: self * x.a,
            b: self * x.b,
            c: self * x.c,
        }
    }
}
