use super::coord3d::*;
use super::vector3d::*;

pub struct Ray {
    origin: Coord3D,
    direction: Vector3D
}

impl Ray {
    pub fn new(origin: Coord3D, direction: Vector3D) -> Ray {
        Ray {
            origin: origin,
            direction: direction.normalize()
        }
    }
    
    pub fn origin(&self) -> Coord3D {
        self.origin
    }
    
    pub fn direction(&self) -> Vector3D {
        self.direction
    }
}