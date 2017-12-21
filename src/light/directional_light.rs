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
        self.color * point.normal().dot(self.direction)
    }

    fn direction(&self, point: &HitPoint) -> Vector3D {
        self.direction
    }
}
