use super::*;
use super::brdf::*;
use light::*;
use utility::*;
use std::fmt;

pub struct Matte {
    ambient: Lambertian,
    diffuse: Lambertian,
}

impl Matte {
    pub fn new(ka: f64, kd: f64, color: RGBColor) -> Matte {
        Matte { ambient: Lambertian::new(ka, color), diffuse: Lambertian::new(kd, color) }
    }
}

impl Material for Matte {
    fn shade(&self, point: &HitPoint, camera_ray: &Ray, world: &World) -> RGBColor {
        let wo = -camera_ray.direction();
        let mut color = (**world.ambient_light()).incident_radiance_at(point) * self.ambient.rho(point, wo);

        for light in world.lights().iter() {
            let wi = (*light).direction(point);
            //println!("wi: {}", wi);
            let cos = point.normal().dot(wi);

            if cos > 0. {
                color = color + self.diffuse.f(point, wi, wo) * (*light).incident_radiance_at(point);
            }
        }

        //println!("{}", color);
        let c = color.max_to_one();
        c
    }
}
