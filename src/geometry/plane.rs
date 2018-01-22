use super::*;
use utility::*;

pub struct Plane {
    point: Coord3D,
    normal: Vector3D,
    material_name: String
}

impl Plane {
    pub fn new(point: Coord3D, normal: Vector3D, material_name: String) -> Plane {
        Plane {
            point: point,
            normal: normal,
            material_name: material_name
        }
    }
    
    pub fn new_from_dict(dict: &Dictionary) -> Result<Plane, Box<Error>> {
        let mut point_split = dict.get("point").ok_or("point is missing")?.split(",");
        let point = Coord3D::new(point_split.next().unwrap().trim().parse::<f64>()?, point_split.next().unwrap().trim().parse::<f64>()?, point_split.next().unwrap().trim().parse::<f64>()?);
        let mut normal_split = dict.get("normal").ok_or("normal is missing")?.split(",");
        let normal = Vector3D::new(normal_split.next().unwrap().trim().parse::<f64>()?, normal_split.next().unwrap().trim().parse::<f64>()?, normal_split.next().unwrap().trim().parse::<f64>()?);
        let material_name = dict.get("material").ok_or("material is missing").unwrap();

        Ok(Plane::new(point, normal, material_name.to_string()))
    }
}

impl GeometricObject for Plane {
    fn hit(&self, ray: &Ray) -> Option<(HitPoint, f64)> {
        let t = (self.point - ray.origin()).dot(self.normal) / (ray.direction().dot(self.normal));

        if t > KEPSILON {
            return Some((HitPoint::new(ray.origin() + ray.direction() * t, self.normal, self.material_name.clone()), t))
        }

        return None
    }
}
