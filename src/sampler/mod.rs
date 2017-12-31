pub use self::random_sampler::*;

mod random_sampler;

use utility::*;

pub trait Sampler {
    fn sample(&self) -> Vec<Coord2D>;
    fn sample_num(&self) -> i32;
}