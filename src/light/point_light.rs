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
