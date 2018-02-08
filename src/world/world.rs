extern crate minidom;

use utility::*;
use geometry::*;
use tracer::*;
use light::*;
use material::*;
use camera::*;

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
    camera: Box<PinholeCamera>
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
            tracer: Box::new(RayCast::new()),
            camera: Box::new(PinholeCamera::new(40., 1920, 1080, Coord3D::new(0, 0, 200), Coord3D::new(0, 0, 0)))
        }
    }

    pub fn camera(&self) -> &Box<PinholeCamera> {
        &self.camera
    }
    
    pub fn add_object(&mut self, object: Box<GeometricObject>) {
        self.objects.push(object);
    }
    
    pub fn add_light(&mut self, light: Box<Light>) {
        self.lights.push(light);
    }

    pub fn hit_objects(&self, ray: &Ray) -> Option<(HitPoint, f64)> {
        let mut tmin = MAX_DISTANCE;
        let mut sr = None;

        for object in &self.objects {
            let result = object.hit(ray);
            match result {
                Some((x, t)) => if t < tmin {
                    tmin = t;
                    sr = Some((x, t));
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
        let mut camera = None;
        
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
                        "ambient" | "ambientlight" | "ambientoccluder" => ambient_light = Some(light),
                        _ => lights.push(light)
                    }
                }
                "geometry" =>
                for geometry_node in child.children() {
                    geometry.push(geometry_node);
                }
                "camera" => camera = Some(Box::new(PinholeCamera::new_from_dict(child).unwrap())),
                _ => (),
            }
        }

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
            tracer: Box::new(RayCast::new()),
            camera: camera.unwrap()
        }
    }
}

impl Dictionary for minidom::Element {
    fn get(&self, name: &str) -> Option<&str> {
        self.attr(name)
    }
}
