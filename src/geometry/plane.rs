use super::*;
use utility::*;
use material::*;

pub struct Plane {
    point: Coord3D,
    normal: Vector3D,
    material: Box<Material>
}

impl Plane {
    pub fn new(point: Coord3D, normal: Vector3D, material: Box<Material>) -> Plane {
        Plane {
            point: point,
            normal: normal,
            material: material
        }
    }
}

impl GeometricObject for Plane {
    fn hit(&self, ray: &Ray, tmin:&mut f64) -> Option<HitPoint> {
        let t = (self.point - ray.origin()).dot(self.normal) / (ray.direction().dot(self.normal));

        if t > KEPSILON {
            *tmin = t;
            return Some(HitPoint::new(ray.origin() + ray.direction() * t, self.normal, &*self.material))
        }

        return None
    }
}
