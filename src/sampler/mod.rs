pub use self::random_sampler::*;
pub use self::regular_sampler::*;

mod random_sampler;
mod regular_sampler;

use utility::*;

pub trait Sampler {
    fn sample(&self) -> Vec<Coord2D>;
    fn sample_num(&self) -> i32;
}