use super::*;
use utility::*;
use std::error::Error;

pub struct AmbientLight {
    color: RGBColor
}

impl AmbientLight {
    pub fn new(color: RGBColor) -> AmbientLight {
        AmbientLight { color: color }
    }
    
    pub fn new_from_dict(map: &Dictionary) -> Result<AmbientLight, Box<Error>> {
        let color = RGBColor::from_hex(map.get("color").ok_or("color is missing")?)?;

        Ok(AmbientLight::new(color))
    }
}

impl Light for AmbientLight {
    fn color(&self) -> RGBColor {
        self.color
    }

    fn incident_radiance_at(&self, _: &HitPoint, _: &World) -> RGBColor {
        self.color
    }

    fn direction(&self, _: Coord3D) -> Vector3D {
        Vector3D::new(0, 0, 0)
    }

    fn is_in_shadow(&self, _: Coord3D, _: &World) -> bool {
        true
    }
}
