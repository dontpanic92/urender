use super::*;
use utility::*;
use world::*;

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
            Some((point, _)) => world.material(point.material_name()).shade(&point, ray, world),
            None => world.bgcolor(),
        }
    }
}