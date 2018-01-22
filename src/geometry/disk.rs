use super::*;
use utility::*;

pub struct Disk {
    center: Coord3D,
    normal: Vector3D,
    radius: f64,
    material_name: String
}

impl Disk {
    pub fn new(center: Coord3D, normal: Vector3D, radius: f64, material_name: String) -> Disk {
        Disk {
            center: center,
            normal: normal,
            radius: radius,
            material_name: material_name
        }
    }
    
    pub fn new_from_dict(dict: &Dictionary) -> Result<Disk, Box<Error>> {
        let mut center_split = dict.get("center").ok_or("center is missing")?.split(",");
        let center = Coord3D::new(center_split.next().unwrap().trim().parse::<f64>()?, center_split.next().unwrap().trim().parse::<f64>()?, center_split.next().unwrap().trim().parse::<f64>()?);
        let mut normal_split = dict.get("normal").ok_or("normal is missing")?.split(",");
        let normal = Vector3D::new(normal_split.next().unwrap().trim().parse::<f64>()?, normal_split.next().unwrap().trim().parse::<f64>()?, normal_split.next().unwrap().trim().parse::<f64>()?);
        let radius = dict.get("radius").ok_or("radius is missing")?.parse::<f64>()?;
        let material_name = dict.get("material").ok_or("material is missing").unwrap();

        Ok(Disk::new(center, normal, radius, material_name.to_string()))
    }
}

impl GeometricObject for Disk {
    fn hit(&self, ray: &Ray) -> Option<(HitPoint, f64)> {
        let t = (self.center - ray.origin()).dot(self.normal) / (ray.direction().dot(self.normal));

        if t > KEPSILON {
            let hit_coord = ray.origin() + ray.direction() * t;
            if (hit_coord - self.center).norm() <= self.radius {
                return Some((HitPoint::new(hit_coord, self.normal, self.material_name.clone()), t))
            }
        }

        return None
    }
}
