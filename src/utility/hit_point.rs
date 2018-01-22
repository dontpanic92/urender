use utility::*;

pub struct HitPoint {
    coord: Coord3D,
    normal: Vector3D,
    material_name: String
}

impl HitPoint {
    pub fn new(coord: Coord3D, normal: Vector3D, material_name: String) -> HitPoint {
        HitPoint {
            coord: coord,
            normal: normal.normalize(),
            material_name: material_name
        }
    }
    
    pub fn coord(&self) -> Coord3D {
        self.coord
    }

    pub fn normal(&self) -> Vector3D {
        self.normal
    }

    pub fn material_name(&self) -> &String {
        &self.material_name
    }
}
