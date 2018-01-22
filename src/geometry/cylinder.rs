use super::*;
use utility::*;

pub struct Cylinder {
    center1: Coord3D,
    center2: Coord3D,
    radius: f64,
    material_name: String
}

impl Cylinder {
    pub fn new(center1: Coord3D, center2: Coord3D, radius: f64, material_name: String) -> Cylinder {
        Cylinder {
            center1: center1,
            center2: center2,
            radius: radius,
            material_name: material_name
        }
    }
    
    pub fn new_from_dict(dict: &Dictionary) -> Result<Cylinder, Box<Error>> {
        let mut center1_split = dict.get("center1").ok_or("center1 is missing")?.split(",");
        let center1 = Coord3D::new(center1_split.next().unwrap().trim().parse::<f64>()?, center1_split.next().unwrap().trim().parse::<f64>()?, center1_split.next().unwrap().trim().parse::<f64>()?);
        let mut center2_split = dict.get("center2").ok_or("center2 is missing")?.split(",");
        let center2 = Coord3D::new(center2_split.next().unwrap().trim().parse::<f64>()?, center2_split.next().unwrap().trim().parse::<f64>()?, center2_split.next().unwrap().trim().parse::<f64>()?);


        let radius = dict.get("radius").ok_or("radius is missing")?.parse::<f64>()?;
        let material_name = dict.get("material").ok_or("material is missing").unwrap();

        Ok(Cylinder::new(center1, center2, radius, material_name.to_string()))
    }
}

impl GeometricObject for Cylinder {
    fn hit(&self, ray: &Ray) -> Option<(HitPoint, f64)> {
        // Reference: http://mrl.nyu.edu/~dzorin/rend05/lecture2.pdf
        let va = (self.center2 - self.center1).normalize();
        let pa = self.center1;
        let v = ray.direction();
        let p = ray.origin();
        let deltap = p - pa;

        let a = (v - v.dot(va) * va).norm().powi(2);
        let b = (v - v.dot(va) * va).dot((deltap - deltap.dot(va) * va)) * 2.;
        let c = (deltap - deltap.dot(va) * va).norm().powi(2) - self.radius.powi(2);

        let result = |ray: &Ray, t: f64| {
            let hit_vector = ray.direction() * t;
            let hit_coord = ray.origin() + hit_vector;
            let normal = hit_coord - (pa + (hit_vector.dot(va) + deltap.dot(va)) * va);

            if (hit_coord - self.center1).dot(va) > 0. && (hit_coord - self.center2).dot(va) < 0. {
                Some((HitPoint::new(hit_coord, normal, self.material_name.clone()), t))
            } else {
                None
            }
        };

        let disc = b * b - 4. * a * c;
        let mut candidate = None;
        if disc > 0. {
            let e = disc.sqrt();
            let denom = 2. * a;
            let mut t = (-b - e) / denom;
            if t > KEPSILON {
                candidate = result(ray, t);
            } else {
                t = (-b + e) / denom;
                
                if t > KEPSILON {
                    candidate = result(ray, t);
                }
            }
        }

        let select_min = |c1: Option<(HitPoint, f64)>, c2: Option<(HitPoint, f64)>, c3: Option<(HitPoint, f64)>| {
            let select = |a: Option<(HitPoint, f64)>, b: Option<(HitPoint, f64)>| {
                if a.is_none() {
                    b
                } else if b.is_none() {
                    a
                } else {
                    if a.as_ref().unwrap().1 < b.as_ref().unwrap().1 {
                        a
                    } else {
                        b
                    }
                }
            };
            select(select(c1, c2), c3)
        };

        let cap1 = Disk::new(self.center1, -va, self.radius, self.material_name.clone());
        let cap2 = Disk::new(self.center2, va, self.radius, self.material_name.clone());
        let hitresult1 = cap1.hit(ray);
        let hitresult2 = cap2.hit(ray);
        
        select_min(candidate, hitresult1, hitresult2)
    }
}
