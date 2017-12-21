use super::*;
use utility::*;
use bmp::Pixel;

pub struct PinholeCamera {
    vp: ViewPlane,
    eye: Coord3D
}

impl PinholeCamera {
    pub fn new() -> PinholeCamera
    {
        PinholeCamera { vp: ViewPlane::new(), eye: Coord3D::new(0, 0, 200) }
    }
    
    pub fn view_plane(&self) -> &ViewPlane {
        &self.vp
    }

    pub fn position(&self) -> &Coord3D {
        &self.eye
    }
}

impl Camera for PinholeCamera {
    fn render(&self, world: &World, tracer: &Tracer) -> Image {
        let mut img = Image::new(self.vp.hres(), self.vp.vres());
        let zw = 150.0;

        for (x, y) in img.coordinates() {
            let rx = self.vp.pixel_size() as f64 * (x as f64 - (self.vp.hres() as f64 - 1.) / 2.);
            let ry = self.vp.pixel_size() as f64 * ((self.vp.vres() - y) as f64 -(self.vp.vres() as f64 - 1.) / 2.);
            let ray = Ray::new(self.eye, Vector3D::new(rx, ry, -zw).normalize());
            let color = tracer.trace_ray(&ray, &world);
            
            img.set_pixel(x, y, Pixel { r: color.r_u8(), g: color.g_u8(), b: color.b_u8() } );
        }
        img
    }
}