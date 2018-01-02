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
        let phong_red = Box::new(Phong::new(0.25, 1., 1., 20., RED));
        let phong_green = Box::new(Phong::new(0.25, 1., 1., 20., GREEN));
        let phong_blue = Box::new(Phong::new(0.25, 1., 1., 20., RED));
        let matte_red = Box::new(Matte::new(0.25, 1., RED));
        let matte_green = Box::new(Matte::new(0.25, 1., GREEN));
        let matte_gray = Box::new(Matte::new(0.25, 1., RGBColor::new(0.2, 0.2, 0.2)));

        self.add_object(Box::new(Sphere::new(Coord3D::new(0, 20, 0), 65., phong_red)));
        self.add_object(Box::new(Sphere::new(Coord3D::new(0, -40, 0), 85., phong_green)));
        self.add_object(Box::new(Plane::new(Coord3D::new(0, -40, 0), Vector3D::new(0, 1, 0), matte_gray)));
        // self.add_light(Box::new(DirectionalLight::new(Vector3D::new(-1, -1, 0), WHITE)));
        self.add_light(Box::new(PointLight::new(Coord3D::new(200, 200, 100), WHITE)));
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

    pub fn up() -> Vector3D {
        Vector3D::new(0, 1, 0)
    }
}
