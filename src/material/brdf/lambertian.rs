use super::*;
use std::f64::consts;

pub struct Lambertian {
    kd: f64,
    color: RGBColor
}

impl Lambertian {
    pub fn new(kd: f64, color: RGBColor) -> Lambertian {
        Lambertian{ kd, color }
    }
}

impl BRDF for Lambertian {
    fn f(&self, _: &HitPoint, _: Vector3D, _: Vector3D) -> RGBColor {
        self.color * self.kd * consts::FRAC_1_PI
    }

    fn rho(&self, _: &HitPoint, _: Vector3D) -> RGBColor {
        self.color * self.kd
    }
}

