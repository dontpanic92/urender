use super::*;
use utility::*;
use sampler::*;
use bmp::Pixel;
use std::error::Error;
use std::io;
use std::io::Write;

pub struct PinholeCamera {
    vfov: f64,
    vres: u32,
    hres: u32,
    eye: Coord3D,
    looking_at: Coord3D,
    sampler: Box<Sampler>
}

impl PinholeCamera {
    pub fn new(vfov: f64, vres: u32, hres: u32, eye: Coord3D, lookat: Coord3D) -> PinholeCamera {
        PinholeCamera { vfov: vfov, vres: vres, hres: hres, eye: eye, looking_at: lookat, sampler: Box::new(MultiJitteredSampler::new(2, 2)) }
    }

    pub fn new_from_dict(dict: &Dictionary) -> Result<PinholeCamera, Box<Error>> {
        let mut position_split = dict.get("position").ok_or("position is missing")?.split(",");
        let position = Coord3D::new(position_split.next().unwrap().trim().parse::<f64>()?, position_split.next().unwrap().trim().parse::<f64>()?, position_split.next().unwrap().trim().parse::<f64>()?);
        let mut lookat_split = dict.get("lookat").ok_or("lookat is missing")?.split(",");
        let lookat = Coord3D::new(lookat_split.next().unwrap().trim().parse::<f64>()?, lookat_split.next().unwrap().trim().parse::<f64>()?, lookat_split.next().unwrap().trim().parse::<f64>()?);

        let vfov = dict.get("vfov").ok_or("vfov is missing")?.parse::<f64>()?;
        let vres = dict.get("vres").ok_or("vres is missing")?.parse::<u32>()?;
        let hres = dict.get("hres").ok_or("hres is missing")?.parse::<u32>()?;

        Ok(PinholeCamera::new(vfov, vres, hres, position, lookat))
    }
}

impl Camera for PinholeCamera {
    fn render(&self, world: &World, tracer: &Tracer) -> Image {
        let mut img = Image::new(self.hres, self.vres);
        let up = World::up();
        let local_front = (self.looking_at() - self.position()).normalize();
        let local_left = up.cross(local_front).normalize();
        let local_up = local_front.cross(local_left).normalize();

        // println!("front {} left {} up {}", local_front, local_left, local_up);
        let total = (self.hres * self.vres) as f64;
        for (i, (x, y)) in img.coordinates().enumerate() {
            if i % 10000 == 0 {
                let progress = i as f64 / total;
                print!("Rendering... {:.2}%\r", progress * 100.);
                io::stdout().flush().unwrap();
            }

            let rx = - (x as f64 - (self.hres as f64 - 1.) / 2.);
            let ry = (self.vres - y) as f64 -(self.vres as f64 - 1.) / 2.;
            // let direction_local = Vector3D::new(rx, ry, 150);
            // let direction = (rx * local_left + ry * local_up + 150 * local_front).normalize();
            // Vector3D::new(rx * (local_up.x() + local_front.x() + local_left.x()), ry * (local_up.y() + local_front.y() + local_left.y()), 150. * (local_up.z() + local_front.z() + local_left.z())).normalize();

            let pixel_size = (self.vfov / 2.).to_radians().tan() * 2. / self.vres as f64;
            let mut color = BLACK;
            let samples = self.sampler.sample();
            for coord in samples {
                let new_x = (rx - 0.5 + coord.x()) * pixel_size;
                let new_y = (ry - 0.5 + coord.y()) * pixel_size;
                let direction = (new_x * local_left + new_y * local_up + local_front).normalize();
                let ray = Ray::new(self.eye, direction);
                let sample_color = tracer.trace_ray(&ray, &world);
                color = color + sample_color;
            }

            color = color / self.sampler.sample_num() as f64;
            
            img.set_pixel(x, y, Pixel { r: color.r_u8(), g: color.g_u8(), b: color.b_u8() } );
            // println!("{} {} {} {} {}", x, y, color.r_u8(), color.g_u8(), color.b_u8() );
        }
        img
    }

    fn position(&self) -> Coord3D {
        self.eye
    }
    
    fn set_position(&mut self, position: Coord3D) {
        self.eye = position;
    }

    fn looking_at(&self) -> Coord3D {
        self.looking_at
    }

    fn look_at(&mut self, position: Coord3D) {
        self.looking_at = position;
    }
}