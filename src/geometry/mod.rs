pub use self::sphere::*;
pub use self::plane::*;
pub use self::cylinder::*;
pub use self::disk::*;
pub use self::aabox::*;

mod sphere;
mod plane;
mod cylinder;
mod disk;
mod aabox;

use utility::*;
use std::error::Error;

const KEPSILON: f64 = 0.00001;

pub trait GeometricObject {
    fn hit(&self, ray: &Ray) -> Option<(HitPoint, f64)>;
}

pub fn create_geometry(geometry_type: &str, dict: &Dictionary) -> Result<Box<GeometricObject>, Box<Error>> {
    match geometry_type.to_lowercase().as_str() {
        "plane" => Ok(Box::new(self::Plane::new_from_dict(dict)?)),
        "sphere" => Ok(Box::new(self::Sphere::new_from_dict(dict)?)),
        "disk" => Ok(Box::new(self::Disk::new_from_dict(dict)?)),
        "cylinder" => Ok(Box::new(self::Cylinder::new_from_dict(dict)?)),
        "aabox" | "aab" => Ok(Box::new(self::AABox::new_from_dict(dict)?)),
        _ => Err(error("Geometry type not supported: ".to_string() + geometry_type)),
    }
}
