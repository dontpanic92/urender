pub use self::matte::*;

mod brdf;
mod matte;

use utility::*;
use world::*;

pub trait Material {
    fn shade(&self, sr: &HitPoint, camera_ray: &Ray, world: &World) -> RGBColor;
}