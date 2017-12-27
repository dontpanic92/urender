use super::*;
use utility::*;
use bmp::Pixel;

pub struct PinholeCamera {
    vp: ViewPlane,
    eye: Coord3D,
    looking_at: Coord3D
}

impl PinholeCamera {
    pub fn new() -> PinholeCamera
    {
        PinholeCamera { vp: ViewPlane::new(), eye: Coord3D::new(100, 100, 100), looking_at: Coord3D::new(0, 0, 0) }
    }
    
    pub fn view_plane(&self) -> &ViewPlane {
        &self.vp
    }
}

impl Camera for PinholeCamera {
    fn render(&self, world: &World, tracer: &Tracer) -> Image {
        let mut img = Image::new(self.vp.hres(), self.vp.vres());
        let up = World::up();
        let local_front = (self.looking_at() - self.position()).normalize();
        let local_left = up.cross(local_front).normalize();
        let local_up = local_front.cross(local_left);
        // let zw = 150.0;

        for (x, y) in img.coordinates() {
            let rx = - self.vp.pixel_size() as f64 * (x as f64 - (self.vp.hres() as f64 - 1.) / 2.);
            let ry = self.vp.pixel_size() as f64 * ((self.vp.vres() - y) as f64 -(self.vp.vres() as f64 - 1.) / 2.);
            let direction_local = Vector3D::new(rx, ry, 150);
            let direction = (rx * local_left + ry * local_up + 150 * local_front).normalize();
            // Vector3D::new(rx * (local_up.x() + local_front.x() + local_left.x()), ry * (local_up.y() + local_front.y() + local_left.y()), 150. * (local_up.z() + local_front.z() + local_left.z())).normalize();

            let ray = Ray::new(self.eye, direction);
            let color = tracer.trace_ray(&ray, &world);
            
            img.set_pixel(x, y, Pixel { r: color.r_u8(), g: color.g_u8(), b: color.b_u8() } );
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