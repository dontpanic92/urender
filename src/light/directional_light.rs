use super::*;
use utility::*;
use std::error::Error;

pub struct DirectionalLight {
    direction: Vector3D,
    color: RGBColor
}

impl DirectionalLight {
    pub fn new(direction: Vector3D, color: RGBColor) -> DirectionalLight {
        DirectionalLight { direction: -direction.normalize(), color: color }
    }    
    
    pub fn new_from_dict(map: &Dictionary) -> Result<DirectionalLight, Box<Error>> {
        let mut split = map.get("direction").ok_or("direction is missing")?.split(",");
        let direction = Vector3D::new(split.next().unwrap().trim().parse::<f64>()?, split.next().unwrap().trim().parse::<f64>()?, split.next().unwrap().trim().parse::<f64>()?);
        let color = RGBColor::from_hex(map.get("color").ok_or("color is missing")?)?;

        Ok(DirectionalLight::new(direction, color))
    }
}

impl Light for DirectionalLight {
    fn color(&self) -> RGBColor {
        self.color
    }

    
    fn incident_radiance_at(&self, _: &HitPoint, _: &World) -> RGBColor {
        self.color
        // point.normal().dot(self.direction)
    }

    fn direction(&self, _: Coord3D) -> Vector3D {
        self.direction
    }

    fn is_in_shadow(&self, coord: Coord3D, world: &World) -> bool {
        let direction = self.direction(coord);
        let shadow_ray = Ray::new(coord, direction);
        match world.hit_objects(&shadow_ray) {
            None => false,
            Some(_) => true
        }
    }
}
