extern crate minidom;

use utility::*;
use geometry::*;
use tracer::*;
use light::*;
use material::*;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use self::minidom::Element;

pub struct World {
    bgcolor: RGBColor,
    objects: Vec<Box<GeometricObject>>,
    lights: Vec<Box<Light>>,
    materials: HashMap<String, Box<Material>>,
    ambient_light: Box<Light>,
    tracer: Box<Tracer>,
}

const MAX_DISTANCE: f64 = 99999999.;

impl World {
    pub fn new() -> World {
        World {
            bgcolor: BLACK,
            objects: Vec::new(),
            lights: Vec::new(),
            materials: HashMap::new(),
            ambient_light: Box::new(AmbientLight::new(WHITE)),
            tracer: Box::new(RayCast::new())
        }
    }
    
    pub fn build(&mut self) {
        let phong_red = Box::new(Phong::new(0.2, 2.5, 1., 10., RED));
        let phong_green = Box::new(Phong::new(0.2, 2.5, 1., 10., GREEN));
        let phong_white = Box::new(Phong::new(0.1, 1., 1., 5., WHITE));
        let matte_red = Box::new(Matte::new(0.25, 1., RED));
        let matte_green = Box::new(Matte::new(0.25, 1., GREEN));
        let matte_white = Box::new(Matte::new(0.2, 2.5, WHITE));

        // self.add_object(Box::new(Sphere::new(Coord3D::new(0, 20, 0), 65., phong_red)));
        // self.add_object(Box::new(Sphere::new(Coord3D::new(0, -40, 0), 85., phong_green)));
        // self.add_object(Box::new(Plane::new(Coord3D::new(0, -40, 0), Vector3D::new(0, 1, 0), matte_white)));
        // self.add_light(Box::new(DirectionalLight::new(Vector3D::new(-1, -1, 0), WHITE)));  
        // self.add_light(Box::new(PointLight::new(Coord3D::new(200, 200, 100), WHITE)));
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
            let result = object.hit(ray, &mut t, &self);
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

    pub fn ambient_light(&self) -> &Box<Light> {
        &self.ambient_light
    }

    pub fn lights(&self) -> &Vec<Box<Light>> {
        &self.lights
    }

    pub fn up() -> Vector3D {
        Vector3D::new(0, 1, 0)
    }

    pub fn material(&self, material_name: &String) -> &Material {
        &**self.materials.get(material_name).unwrap()
    }

    pub fn load_scene(xml_scene_path: &str) -> World {
        let mut file = File::open(xml_scene_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        let root: Element = content.parse().unwrap();
        let mut materials = HashMap::new();
        let mut lights = Vec::new();
        let mut geometry = Vec::new();
        let mut geometry_objects = Vec::new();
        let mut ambient_light = None;
        
        for child in root.children() {
             match child.name().to_lowercase().as_str() {
                "materials"=>
                for material_node in child.children() {
                    let material_type = material_node.name();
                    let material = create_material(material_type, material_node).unwrap();
                    let name = material_node.attr("name").unwrap().to_string();
                    materials.insert(name, material);
                },
                "lights" => 
                for light_node in child.children() {
                    let light_type = light_node.name();
                    let light = create_light(light_type, light_node).unwrap();
                    match light_type.to_lowercase().as_str() {
                        "ambient" | "ambientlight" => ambient_light = Some(light),
                        _ => lights.push(light)
                    }
                }
                "geometry" =>
                for geometry_node in child.children() {
                    geometry.push(geometry_node);
                }
                _ => (),
            }
        }

        println!("{}", lights.len());

        for g in geometry {
            let material_name = g.attr("material").unwrap();
            materials.get(&material_name.to_string()).unwrap();

            let object = create_geometry(g.name(), g).unwrap();
            geometry_objects.push(object);
        }

        World {
            bgcolor: BLACK,
            objects: geometry_objects,
            lights: lights,
            materials: materials,
            ambient_light: ambient_light.unwrap(),
            tracer: Box::new(RayCast::new())
        }
    }
}

impl Dictionary for minidom::Element {
    fn get(&self, name: &str) -> Option<&str> {
        self.attr(name)
    }
}
