pub use self::random_sampler::*;
pub use self::regular_sampler::*;
use std::f64::consts;

mod random_sampler;
mod regular_sampler;

use utility::*;

pub trait Sampler {
    fn sample(&self) -> Vec<Coord2D>;
    fn sample_num(&self) -> i32;

    fn sample_hemisphere(&self) -> Vec<Coord3D> {
        let samples = self.sample();
        let mut hemisphere_samples = vec![];

        for sample in samples {
            let cos_phi = (2. * consts::PI * sample.x()).cos();
            let sin_phi = (2. * consts::PI * sample.x()).sin();
            let cos_theta = (1. - sample.y()).powf(1. / (consts::E + 1.));
            let sin_theta = (1. - cos_theta * cos_theta).sqrt();
            hemisphere_samples.push(Coord3D::new(sin_theta * cos_phi, sin_theta * sin_phi, cos_theta));
        }

        hemisphere_samples
    }
}