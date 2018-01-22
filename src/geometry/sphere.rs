use super::*;
use utility::*;

pub struct Sphere {
    center: Coord3D,
    radius: f64,
    material_name: String
}

impl Sphere {
    pub fn new(center: Coord3D, radius: f64, material_name: String) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material_name: material_name
        }
    }

    pub fn new_from_dict(dict: &Dictionary) -> Result<Sphere, Box<Error>> {
        let mut split = dict.get("center").ok_or("center is missing")?.split(",").peekable();
        let center = Coord3D::new(split.next().unwrap().trim().parse::<f64>()?, split.next().unwrap().trim().parse::<f64>()?, split.next().unwrap().trim().parse::<f64>()?);
        let radius = dict.get("radius").ok_or("radius is missing")?.parse::<f64>()?;
        let material_name = dict.get("material").ok_or("material is missing").unwrap();

        Ok(Sphere::new(center, radius, material_name.to_string()))
    }
}

impl GeometricObject for Sphere {
    fn hit(&self, ray: &Ray) -> Option<(HitPoint, f64)> {
        let temp = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2. * temp.dot(ray.direction()); 
        let c = temp.dot(temp) - self.radius * self.radius;
        let disc = b * b - 4. * a * c;
        if disc > 0. {
            let e = disc.sqrt();
            let denom = 2. * a;
            let mut t = (-b - e) / denom;
            
            if t > KEPSILON {
                return Some((HitPoint::new(ray.origin() + ray.direction() * t, temp + t * ray.direction(), self.material_name.clone()), t));
            }
            
            t = (-b + e) / denom;
            
            if t > KEPSILON {
                return Some((HitPoint::new(ray.origin() + ray.direction() * t, temp + t * ray.direction(), self.material_name.clone()), t));
            }
        }

        return None
    }
}
