pub use self::matte::*;
pub use self::phong::*;

mod brdf;
mod matte;
mod phong;

use utility::*;
use world::*;

pub trait Material {
    fn shade(&self, sr: &HitPoint, camera_ray: &Ray, world: &World) -> RGBColor;
}