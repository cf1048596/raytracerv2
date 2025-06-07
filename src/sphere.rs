use std::rc::Rc;

use crate::ray::HitRecord;
use crate::ray::Hittable;
use crate::ray::Ray;
use crate::{
    interval::Interval,
    ray::{Scatter, SetFaceNormal},
    vec3::{dot, Point3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Scatter>>,
}

impl Sphere {
    pub fn new(_center: Point3, _radius: f64, material: Rc<dyn Scatter>) -> Sphere {
        Sphere {
            center: _center,
            radius: _radius.max(0_f64),
            mat: Some(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.dir().get_len_squared();
        let h = dot(&ray.dir(), &oc);
        let c = oc.get_len_squared() - self.radius.powi(2);
        let discriminant = h.powi(2) - a * c;
        match discriminant {
            d if d < 0_f64 => false,
            _ => {
                let sqrtd = discriminant.sqrt();
                let mut root = (h - sqrtd) / a;
                if !ray_t.surrounds(root) {
                    root = (h + sqrtd) / a;
                    if !ray_t.surrounds(root) {
                        return false;
                    }
                }
                rec.t = root;
                rec.p = ray.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(ray, &outward_normal);
                rec.mat = self.mat.clone();
                true
            }
        }
    }
}
