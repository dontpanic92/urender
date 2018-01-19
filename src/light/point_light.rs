use super::*;
use utility::*;
use std::error::Error;

pub struct PointLight {
    coord: Coord3D,
    color: RGBColor
}

impl PointLight {
    pub fn new(coord: Coord3D, color: RGBColor) -> PointLight {
        PointLight { coord: coord, color: color }
    }

    pub fn coord(&self) -> Coord3D {
        self.coord
    }
    
    pub fn new_from_dict(map: &Dictionary) -> Result<PointLight, Box<Error>> {
        let mut split = map.get("coord").ok_or("coord is missing")?.split(",");
        let coord = Coord3D::new(split.next().unwrap().trim().parse::<f64>()?, split.next().unwrap().trim().parse::<f64>()?, split.next().unwrap().trim().parse::<f64>()?);
        let color = RGBColor::from_hex(map.get("color").ok_or("color is missing")?)?;

        Ok(PointLight::new(coord, color))
    }
}

impl Light for PointLight {
    fn color(&self) -> RGBColor {
        self.color
    }
    
    fn incident_radiance_at(&self, point: &HitPoint) -> RGBColor {
        self.color 
        // point.normal().dot((self.direction(point.coord())))
    }
    
    fn direction(&self, coord: Coord3D) -> Vector3D {
        (self.coord - coord).normalize()
    }

    fn is_in_shadow(&self, coord: Coord3D, world: &World) -> bool {
        let direction = self.direction(coord);
        let shadow_ray = Ray::new(coord, direction);
        match world.hit_objects(&shadow_ray) {
            None => false,
            Some(x) => true
        }
    }
}
