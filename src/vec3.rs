use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Neg;
use std::ops::Sub;

use crate::helper::random_f64;
use crate::helper::random_f64_range;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn new_empty() -> Self {
        Self {
            e: [0.into(), 0.into(), 0.into()],
        }
    }

    pub fn get(&self, i: usize) -> f64 {
        self.e[i]
    }

    pub fn get_mut(&mut self, i: usize) -> &mut f64 {
        &mut self.e[i]
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn get_len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn get_len(&self) -> f64 {
        self.get_len_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn random() -> Self {
        Self::new(random_f64(), random_f64(), random_f64())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            random_f64_range(min, max),
            random_f64_range(min, max),
            random_f64_range(min, max),
        )
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.e[0] *= scalar;
        self.e[1] *= scalar;
        self.e[2] *= scalar;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self *= 1_f64 / scalar
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, othervec: Vec3) -> Self {
        Self {
            e: [
                self.e[0] + othervec.e[0],
                self.e[1] + othervec.e[1],
                self.e[2] + othervec.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, othervec: Vec3) -> Self {
        Self {
            e: [
                self.e[0] - othervec.e[0],
                self.e[1] - othervec.e[1],
                self.e[2] - othervec.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, othervec: Vec3) -> Self {
        Self {
            e: [
                self.e[0] * othervec.e[0],
                self.e[1] * othervec.e[1],
                self.e[2] * othervec.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        (1_f64 / self) * rhs
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Self {
        (1_f64 / scalar) * self
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    )
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    *u / u.get_len()
}

pub fn random_unit_vector() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1_f64, 1_f64);
        let lensq = p.get_len_squared();
        if lensq <= 1_f64 && 1e-160_f64 < lensq {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    match dot(&on_unit_sphere, normal) {
        n if n > 0.0 => on_unit_sphere, //same hemisphere as the normal
        _ => -on_unit_sphere,
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2_f64 * dot(v, n) * *n
}

pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = dot(&(-*uv), n).min(1_f64);
    let r_out_perpen = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1.0 - r_out_perpen.get_len_squared()).abs().sqrt()) * *n;
    r_out_perpen + r_out_parallel
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_f64_range(-1_f64, 1_f64),
            random_f64_range(-1_f64, 1_f64),
            0_f64,
        );
        if p.get_len_squared() < 1_f64 {
            return p;
        };
    }
}
