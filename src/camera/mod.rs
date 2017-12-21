pub use self::pinhole_camera::*;
pub use self::orthographic_camera::*;
pub use self::view_plane::*;

mod pinhole_camera;
mod view_plane;
mod orthographic_camera;

use tracer::*;
use world::*;
use bmp::{Image};

pub trait Camera {
    fn render(&self, world: &World, tracer: &Tracer) -> Image;
}