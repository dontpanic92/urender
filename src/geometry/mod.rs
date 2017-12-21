pub use self::sphere::*;
pub use self::plane::*;

mod sphere;
mod plane;

use utility::*;


const KEPSILON: f64 = 0.00001;

pub trait GeometricObject {
    fn hit(&self, ray: &Ray, tmin: &mut f64,) -> Option<HitPoint>;
}

