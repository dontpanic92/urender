use utility::*;
use material::*;

pub struct HitPoint<'a> {
    coord: Coord3D,
    normal: Vector3D,
    material: &'a Material
}

impl<'a> HitPoint<'a> {
    pub fn new(coord: Coord3D, normal: Vector3D, material: &'a Material) -> HitPoint<'a> {
        HitPoint {
            coord: coord,
            normal: normal.normalize(),
            material: material
        }
    }
    
    pub fn coord(&self) -> Coord3D {
        self.coord
    }

    pub fn normal(&self) -> Vector3D {
        self.normal
    }

    pub fn material(&self) -> &'a Material {
        self.material
    }
}
