pub use self::lambertian::*;

mod lambertian;
use utility::*;

pub trait BRDF {
    fn f(&self, sr: &HitPoint, wi: Vector3D, wo: Vector3D) -> RGBColor;
    fn rho(&self, sr: &HitPoint, wo: Vector3D) -> RGBColor;
}