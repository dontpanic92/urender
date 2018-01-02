use super::*;
use utility::*;

pub struct DirectionalLight {
    direction: Vector3D,
    color: RGBColor
}

impl DirectionalLight {
    pub fn new(direction: Vector3D, color: RGBColor) -> DirectionalLight {
        DirectionalLight { direction: -direction.normalize(), color: color }
    }
}

impl Light for DirectionalLight {
    fn color(&self) -> RGBColor {
        self.color
    }

    
    fn incident_radiance_at(&self, point: &HitPoint) -> RGBColor {
        self.color
        // point.normal().dot(self.direction)
    }

    fn direction(&self, coord: Coord3D) -> Vector3D {
        self.direction
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
