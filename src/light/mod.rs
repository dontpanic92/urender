pub use self::directional_light::*;
pub use self::ambient_light::*;
pub use self::ambient_occluder::*;
pub use self::point_light::*;

mod directional_light;
mod ambient_light;
mod ambient_occluder;
mod point_light;

use utility::*;
use world::*;
use std::error::Error;

pub trait Light {
    fn color(&self) -> RGBColor;
    fn incident_radiance_at(&self, point: &HitPoint, world: &World) -> RGBColor;
    fn direction(&self, coord: Coord3D) -> Vector3D;
    fn is_in_shadow(&self, coord: Coord3D, world: &World) -> bool;
}

pub fn create_light(light_type: &str, dict: &Dictionary) -> Result<Box<Light>, Box<Error>> {
    match light_type.to_lowercase().as_str() {
        "ambient" | "ambientlight" => Ok(Box::new(self::AmbientLight::new_from_dict(dict)?)),
        "ambientoccluder" => Ok(Box::new(self::AmbientOccluder::new_from_dict(dict)?)),
        "directional" | "directionallight" => Ok(Box::new(self::DirectionalLight::new_from_dict(dict)?)),
        "point" | "pointlight" => Ok(Box::new(self::PointLight::new_from_dict(dict)?)),
        _ => Err(error("Light type not supported: ".to_string() + light_type)),
    }
}

