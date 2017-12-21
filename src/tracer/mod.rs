pub use self::ray_cast::*;

mod ray_cast;

use utility::*;
use world::*;

pub trait Tracer {
    fn trace_ray(&self, ray: &Ray, world: &World) -> RGBColor;
}
