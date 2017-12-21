use super::*;
use world::*;
use utility::*;
use tracer::*;
use bmp::Pixel;

pub struct OrthographicCamera {
    vp: ViewPlane,
    direction: Vector3D
}

impl OrthographicCamera {
    pub fn new() -> OrthographicCamera
    {
        OrthographicCamera { vp: ViewPlane::new(), direction: Vector3D::new(0, 0, -1) }
    }
    
    pub fn view_plane(&self) -> &ViewPlane {
        &self.vp
    }

    pub fn direction(&self) -> &Vector3D {
        &self.direction
    }
}

impl Camera for OrthographicCamera {
    fn render(&self, world: &World, tracer: &Tracer) -> Image {
        let mut img = Image::new(self.vp.hres(), self.vp.vres());
        let zw = 100.0;

        for (x, y) in img.coordinates() {
            let rx = self.vp.pixel_size() as f64 * (x as f64 - (self.vp.hres() as f64 - 1.) / 2.);
            let ry = self.vp.pixel_size() as f64 * ((self.vp.vres() - y) as f64 -(self.vp.vres() as f64 - 1.) / 2.);
            let ray = Ray::new(Coord3D::new(rx, ry, zw), Vector3D::new(0, 0, -1));
            let color = tracer.trace_ray(&ray, &world);
            
            img.set_pixel(x, y, Pixel { r: color.r_u8(), g: color.g_u8(), b: color.b_u8() } );
        }

        img
    }
}
