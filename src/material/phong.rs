use super::*;
use super::brdf::*;
use light::*;
use utility::*;

pub struct Phong {
    ambient: Lambertian,
    diffuse: Lambertian,
    specular: GlossySpecular
}

impl Phong {
    pub fn new(ka: f64, kd: f64, ks: f64, e: f64, color: RGBColor) -> Phong {
        Phong { ambient: Lambertian::new(ka, color), diffuse: Lambertian::new(kd, color), specular: GlossySpecular::new(ks, e, color) }
    }
}

impl Material for Phong {
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
                color = color + (self.diffuse.f(point, wi, wo) + self.specular.f(point, wi, wo)) * (*light).incident_radiance_at(point) * cos;
            }
        }

        let c = color.max_to_one();
        c
    }
}
