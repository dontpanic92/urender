use super::*;
use utility::*;

pub struct AmbientLight {
    color: RGBColor
}

impl AmbientLight {
    pub fn new(color: RGBColor) -> AmbientLight {
        AmbientLight { color: color }
    }
}

impl Light for AmbientLight {
    fn color(&self) -> RGBColor {
        self.color
    }

    fn incident_radiance_at(&self, point: &HitPoint) -> RGBColor {
        self.color
    }

    fn direction(&self, coord: Coord3D) -> Vector3D {
        Vector3D::new(0, 0, 0)
    }

    fn is_in_shadow(&self, point: Coord3D, world: &World) -> bool {
        true
    }
}
