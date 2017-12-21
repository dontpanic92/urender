use utility::*;
use geometry::*;
use tracer::*;
use light::*;
use material::*;

pub struct World {
    bgcolor: RGBColor,
    objects: Vec<Box<GeometricObject>>,
    lights: Vec<Box<Light>>,
    ambient_light: Box<AmbientLight>,
    tracer: Box<Tracer>,
}

const MAX_DISTANCE: f64 = 99999999.;

impl World {
    pub fn new() -> World {
        World {
            bgcolor: BLACK,
            objects: Vec::new(),
            lights: Vec::new(),
            ambient_light: Box::new(AmbientLight::new(WHITE)),
            tracer: Box::new(RayCast::new())
        }
    }
    
    pub fn build(&mut self) {
        self.add_object(Box::new(Sphere::new(Coord3D::new(0, 20, 0), 65., Box::new(Matte::new(0.1, 1., RGBColor::new_u8(255, 0, 0))))));
        self.add_object(Box::new(Sphere::new(Coord3D::new(0, -40, 0), 85., Box::new(Matte::new(0.1, 1., RGBColor::new_u8(0, 255, 0))))));
        self.add_light(Box::new(DirectionalLight::new(Vector3D::new(-1, -1, 0), WHITE)));
    }
    
    pub fn add_object(&mut self, object: Box<GeometricObject>) {
        self.objects.push(object);
    }
    
    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    pub fn hit_objects(&self, ray: &Ray) -> Option<HitPoint> {
        let mut t = 0f64;
        let mut tmin = MAX_DISTANCE;
        let mut sr = None;

        for object in &self.objects {
            let result = object.hit(ray, &mut t);
            match result {
                Some(x) => if t < tmin {
                    tmin = t;
                    sr = Some(x);
                },
                None => {}
            }
        }
        
        sr
    }
    
    pub fn bgcolor(&self) -> RGBColor{
        self.bgcolor
    }

    pub fn ambient_light(&self) -> &Box<AmbientLight> {
        &self.ambient_light
    }

    pub fn lights(&self) -> &Vec<Box<Light>> {
        &self.lights
    }
}
