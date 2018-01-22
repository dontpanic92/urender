use super::*;
use std::f64;

pub struct GlossySpecular {
    ks: f64,
    e: f64,
    color: RGBColor
}

impl GlossySpecular {
    pub fn new(ks: f64, e: f64, color: RGBColor) -> GlossySpecular {
        GlossySpecular{ ks, e, color }
    }

    pub fn f_with_color(&self, point: &HitPoint, wi: Vector3D, wo: Vector3D, cs: RGBColor) -> RGBColor {
        let reflection = 2. * point.normal().dot(wi) * point.normal() - wi;
        let alpha = reflection.dot(wo);
        if alpha > 0. {
            cs * self.ks * alpha.powf(self.e)
        } else {
            BLACK
        }
    }
}

impl BRDF for GlossySpecular {
    fn f(&self, point: &HitPoint, wi: Vector3D, wo: Vector3D) -> RGBColor {
        self.f_with_color(point, wi, wo, self.color)
    }

    fn rho(&self, _: &HitPoint, _: Vector3D) -> RGBColor {
        self.color * self.ks
    }
}

