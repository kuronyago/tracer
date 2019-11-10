use crate::hit::Hitable;
use crate::hit::Record;
use crate::ray::Ray;

pub enum World<T: Hitable> {
    Single(T),
    Multiple(Vec<T>),
}

impl Hitable for World<T: Hitable> {
    fn hit(&self, r: &Ray, min: f64, max: f64) -> Option<Record> {
        match self {
            World::Single(object) => object.hit(r, min, max),
            World::Multiple(objs) => {
                let mut closest: Option<Record> = None;

                for object in objs.iter() {
                    if let Some(h) = object.hit(r, min, max) {
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
        }
    }
}
