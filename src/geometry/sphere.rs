use super::*;
use utility::*;
use material::*;

pub struct Sphere {
    center: Coord3D,
    radius: f64,
    material: Box<Material>
}

impl Sphere {
    pub fn new(center: Coord3D, radius: f64, material: Box<Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material
        }
    }
}

impl GeometricObject for Sphere {
    fn hit(&self, ray: &Ray, tmin: &mut f64) -> Option<HitPoint> {
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
                *tmin = t;
                return Some(HitPoint::new(ray.origin() + ray.direction() * t, temp + t * ray.direction(), &*self.material));
            }
            
            t = (-b + e) / denom;
            
            if t > KEPSILON {
                *tmin = t;
                return Some(HitPoint::new(ray.origin() + ray.direction() * t, temp + t * ray.direction(), &*self.material));
            }
        }

        return None
    }
}
