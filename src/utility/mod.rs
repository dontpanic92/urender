pub use self::rgb_color::*;
pub use self::coord2d::*;
pub use self::coord3d::*;
pub use self::vector3d::*;
pub use self::ray::*;
pub use self::hit_point::*;
pub use self::error::*;

mod rgb_color;
mod coord2d;
mod coord3d;
mod vector3d;
mod ray;
mod hit_point;
mod error;

pub trait Dictionary {
    fn get(&self, name: &str) -> Option<&str>;
}

#[cfg(test)]
mod test;