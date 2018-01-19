use super::*;
use utility::*;
use material::*;

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
    
    pub fn new_from_dict(map: &Dictionary) -> Result<Plane, Box<Error>> {
        let mut point_split = map.get("point").ok_or("point is missing")?.split(",");
        let point = Coord3D::new(point_split.next().unwrap().trim().parse::<f64>()?, point_split.next().unwrap().trim().parse::<f64>()?, point_split.next().unwrap().trim().parse::<f64>()?);
        let mut normal_split = map.get("normal").ok_or("normal is missing")?.split(",");
        let normal = Vector3D::new(normal_split.next().unwrap().trim().parse::<f64>()?, normal_split.next().unwrap().trim().parse::<f64>()?, normal_split.next().unwrap().trim().parse::<f64>()?);
        let material_name = map.get("material").ok_or("material is missing").unwrap();

        Ok(Plane::new(point, normal, material_name.to_string()))
    }
}

impl GeometricObject for Plane {
    fn hit(&self, ray: &Ray, tmin:&mut f64, world: &World) -> Option<HitPoint> {
        let t = (self.point - ray.origin()).dot(self.normal) / (ray.direction().dot(self.normal));

        if t > KEPSILON {
            *tmin = t;
            return Some(HitPoint::new(ray.origin() + ray.direction() * t, self.normal, self.material_name.clone()))
        }

        return None
    }
}
