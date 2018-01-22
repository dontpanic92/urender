pub use self::matte::*;
pub use self::phong::*;

mod brdf;
mod matte;
mod phong;

use utility::*;
use world::*;
use std::error::Error;

pub trait Material {
    fn shade(&self, sr: &HitPoint, camera_ray: &Ray, world: &World) -> RGBColor;
}

pub fn create_material(material_type: &str, dict: &Dictionary) -> Result<Box<Material>, Box<Error>> {
    match material_type.to_lowercase().as_str() {
        "matte" => Ok(Box::new(self::Matte::new_from_dict(dict)?)),
        "phong" => Ok(Box::new(self::Phong::new_from_dict(dict)?)),
        _ => Err(error("Material type not supported: ".to_string() + material_type)),
    }
}
