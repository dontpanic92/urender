pub use self::lambertian::*;
pub use self::glossy_specular::*;

mod lambertian;
mod glossy_specular;

use utility::*;

pub trait BRDF {
    fn f(&self, point: &HitPoint, wi: Vector3D, wo: Vector3D) -> RGBColor;
    fn rho(&self, point: &HitPoint, wo: Vector3D) -> RGBColor;
}