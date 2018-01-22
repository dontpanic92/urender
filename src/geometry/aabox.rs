use super::*;
use utility::*;

pub struct AABox {
    pmin: Coord3D,
    pmax: Coord3D,
    material_name: String
}

impl AABox {
    pub fn new(p1: Coord3D, p2: Coord3D, material_name: String) -> AABox {
        let minx = p1.x().min(p2.x());
        let maxx = p1.x().max(p2.x());
        let miny = p1.y().min(p2.y());
        let maxy = p1.y().max(p2.y());
        let minz = p1.z().min(p2.z());
        let maxz = p1.z().max(p2.z());

        AABox {
            pmin: Coord3D::new(minx, miny, minz),
            pmax: Coord3D::new(maxx, maxy, maxz),
            material_name: material_name
        }
    }
    
    pub fn new_from_dict(dict: &Dictionary) -> Result<AABox, Box<Error>> {
        let mut coord1_split = dict.get("coord1").ok_or("coord1 is missing")?.split(",");
        let coord1 = Coord3D::new(coord1_split.next().unwrap().trim().parse::<f64>()?, coord1_split.next().unwrap().trim().parse::<f64>()?, coord1_split.next().unwrap().trim().parse::<f64>()?);
        let mut coord2_split = dict.get("coord2").ok_or("coord2 is missing")?.split(",");
        let coord2 = Coord3D::new(coord2_split.next().unwrap().trim().parse::<f64>()?, coord2_split.next().unwrap().trim().parse::<f64>()?, coord2_split.next().unwrap().trim().parse::<f64>()?);

        let material_name = dict.get("material").ok_or("material is missing").unwrap();

        Ok(AABox::new(coord1, coord2, material_name.to_string()))
    }

    pub fn get_normal(face_hit: i32) -> Vector3D {
        match face_hit {
            0 => Vector3D::new(-1, 0, 0),
            1 => Vector3D::new(0, -1, 0),
            2 => Vector3D::new(0, 0, -1),
            3 => Vector3D::new(1, 0, 0),
            4 => Vector3D::new(0, 1, 0),
            5 => Vector3D::new(0, 0, 1),
            _ => panic!("Unknown face_hit")
        }
    }
}

impl GeometricObject for AABox {
    fn hit(&self, ray: &Ray) -> Option<(HitPoint, f64)> {
        let tx_min;
        let ty_min;
        let tz_min;
        let tx_max;
        let ty_max;
        let tz_max;

        macro_rules! calc_t {
            ($x:expr, $func:ident) => {
               ($x.$func() - ray.origin().$func()) / ray.direction().$func();
            };

            ($min_ident:ident, $max_ident:ident, $func:ident) => {
                if ray.direction().$func() > 0. {
                    $min_ident = calc_t!(self.pmin, $func);
                    $max_ident = calc_t!(self.pmax, $func);
                } else {
                    $min_ident = calc_t!(self.pmax, $func);
                    $max_ident = calc_t!(self.pmin, $func);
                }
            };
        }

        calc_t!(tx_min, tx_max, x);
        calc_t!(ty_min, ty_max, y);
        calc_t!(tz_min, tz_max, z);

        let mut face_in;
        let mut face_out;
        let mut t0;
        let mut t1;

        if tx_min > ty_min {
            t0 = tx_min;
            face_in = if ray.direction().x() > 0. {0} else {3};
        } else {
            t0 = ty_min;
            face_in = if ray.direction().y() > 0. {1} else {4};
        }

        if tz_min > t0 {
            t0 = tz_min;
            face_in = if ray.direction().z() > 0. {2} else {5};
        }

        if tx_max < ty_max {
            t1 = tx_max;
            face_out = if ray.direction().x() > 0. {3} else {0};
        } else {
            t1 = ty_max;
            face_out = if ray.direction().y() > 0. {4} else {1};
        }

        if tz_max < t1 {
            t1 = tz_max;
            face_out = if ray.direction().z() > 0. {5} else {2};
        }

        if t0 < t1 && t1 > KEPSILON {
            if t0 > KEPSILON {
                Some((HitPoint::new(ray.origin() + t0 * ray.direction(), AABox::get_normal(face_in), self.material_name.clone()), t0))
            } else {
                Some((HitPoint::new(ray.origin() + t1 * ray.direction(), AABox::get_normal(face_out), self.material_name.clone()), t1))
            }
        } else {
            None
        }
    }
}
