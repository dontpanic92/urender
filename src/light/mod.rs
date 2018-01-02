pub use self::directional_light::*;
pub use self::ambient_light::*;
pub use self::point_light::*;

mod directional_light;
mod ambient_light;
mod point_light;

use utility::*;
use world::*;

pub trait Light {
    fn color(&self) -> RGBColor;
    fn incident_radiance_at(&self, point: &HitPoint) -> RGBColor;
    fn direction(&self, coord: Coord3D) -> Vector3D;
    fn is_in_shadow(&self, coord: Coord3D, world: &World) -> bool;
}
