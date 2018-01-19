use super::*;
use utility::*;
use world::*;
use geometry::*;

pub struct RayCast {
    
}

impl RayCast {
    pub fn new() -> RayCast {
        RayCast {}
    }
}

impl Tracer for RayCast {
    fn trace_ray(&self, ray: &Ray, world: &World) -> RGBColor {
        let sr = world.hit_objects(ray);
        
        match sr {
            Some(x) => world.material(x.material_name()).shade(&x, ray, world),
            None => world.bgcolor(),
        }
    }
}