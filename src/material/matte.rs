use super::*;
use super::brdf::*;
use light::*;
use utility::*;
use std::f64;
use std::error::Error;

pub struct Matte {
    ambient: Lambertian,
    diffuse: Lambertian,
}

impl Matte {
    pub fn new(ka: f64, kd: f64, color: RGBColor) -> Matte {
        Matte { ambient: Lambertian::new(ka, color), diffuse: Lambertian::new(kd, color) }
    }

    pub fn new_from_dict(map: &Dictionary) -> Result<Matte, Box<Error>> {
        let ka = map.get("ka").ok_or("ka is missing")?.parse::<f64>()?;
        let kd = map.get("kd").ok_or("kd is missing")?.parse::<f64>()?;
        let color = RGBColor::from_hex(map.get("color").ok_or("color is missing")?)?;

        Ok(Matte::new(ka, kd, color))
    }
}

impl Material for Matte {
    fn shade(&self, point: &HitPoint, camera_ray: &Ray, world: &World) -> RGBColor {
        let wo = -camera_ray.direction();
        let mut color = (**world.ambient_light()).incident_radiance_at(point) * self.ambient.rho(point, wo);

        for light in world.lights().iter() {
            if light.is_in_shadow(point.coord(), world) {
                continue
            }

            let wi = (*light).direction(point.coord());
            let cos = point.normal().dot(wi);

            if cos > 0. {
                color = color + self.diffuse.f(point, wi, wo) * (*light).incident_radiance_at(point) * cos;
            }
        }

        // println!("{}", color);
        let c = color.max_to_one();
        c
    }
}
