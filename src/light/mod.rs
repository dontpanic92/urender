pub use self::directional_light::*;
pub use self::ambient_light::*;
pub use self::point_light::*;

mod directional_light;
mod ambient_light;
mod point_light;

use utility::*;

pub trait Light {
    fn color(&self) -> RGBColor;
    fn incident_radiance_at(&self, point: &HitPoint) -> RGBColor;
    fn direction(&self, point: &HitPoint) -> Vector3D;
}
