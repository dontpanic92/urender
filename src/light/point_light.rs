use super::*;
use utility::*;

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
}

impl Light for PointLight {
    fn color(&self) -> RGBColor {
        self.color
    }
    
    fn incident_radiance_at(&self, point: &HitPoint) -> RGBColor {
        self.color * point.normal().dot((self.direction(point)))
    }
    
    fn direction(&self, point: &HitPoint) -> Vector3D {
        (self.coord - point.coord()).normalize()
    }
}
