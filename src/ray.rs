use std::{fmt::{write, Pointer}, rc::Rc, sync::Arc};

use crate::{color::Color, interval::{self, Interval}, vec3::{dot, random_unit_vector, reflect, unit_vector, Point3}, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p : Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Scatter>>,
    pub t : f64,
    pub front_face : bool,
}

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, direction: Vec3) -> Self {
        Self {
            origin: orig,
            dir : direction,
        }
    }

    pub fn new_empty() -> Self {
        Self {
            origin: Point3::new_empty(),
            dir: Vec3::new_empty(),
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.dir
    }
}

pub trait Hittable {
    fn hit(&self, ray :&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub trait SetFaceNormal {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3);
}

impl SetFaceNormal for HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.dir(), outward_normal) < 0_f64;
        match self.front_face {
            true => self.normal = *outward_normal,
            false => self.normal = -*outward_normal
        }
    }
}

impl HitRecord {
    pub fn new_empty() -> Self {
        Self {
            p : Point3::new(0_f64, 0_f64, 0_f64),
            normal : Vec3::new(0_f64, 0_f64, 0_f64),
            mat : None,
            t : 0_f64,
            front_face : false,
        }
    }
}

pub struct HittableList {
    objects : Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects : Vec::new()
        }
    }
    pub fn add(&mut self, object:Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray :&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec : HitRecord = HitRecord::new_empty();
        let mut hit_anything : bool = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone()
            }
        }
        hit_anything
    }
}

pub trait Scatter {
    fn scatter(&self, ray_in : &Ray, hit_rec: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool;
}
