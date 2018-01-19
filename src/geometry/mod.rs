pub use self::sphere::*;
pub use self::plane::*;

mod sphere;
mod plane;

use utility::*;
use world::World;
use std::error::Error;

const KEPSILON: f64 = 0.00001;

pub trait GeometricObject {
    fn hit(&self, ray: &Ray, tmin: &mut f64, world: &World) -> Option<HitPoint>;
}

pub fn create_geometry(geometry_type: &str, dict: &Dictionary) -> Result<Box<GeometricObject>, Box<Error>> {
    match geometry_type.to_lowercase().as_str() {
        "plane" => Ok(Box::new(self::Plane::new_from_dict(dict)?)),
        "sphere" => Ok(Box::new(self::Sphere::new_from_dict(dict)?)),
        _ => Err(error("Geometry type not supported: ".to_string() + geometry_type)),
    }
}
