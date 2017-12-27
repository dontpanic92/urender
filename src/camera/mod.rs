pub use self::pinhole_camera::*;
pub use self::orthographic_camera::*;
pub use self::view_plane::*;

mod pinhole_camera;
mod view_plane;
mod orthographic_camera;

use tracer::*;
use world::*;
use utility::*;
use bmp::{Image};

pub trait Camera {
    fn render(&self, world: &World, tracer: &Tracer) -> Image;
    
    fn position(&self) -> Coord3D;
    
    fn set_position(&mut self, position: Coord3D);

    fn looking_at(&self) -> Coord3D;

    fn look_at(&mut self, position: Coord3D);
}