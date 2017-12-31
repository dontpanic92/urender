use super::*;
use utility::*;
use rand;
use rand::Rng;

pub struct RandomSampler {
    sample_num: i32,
}

impl RandomSampler {
    pub fn new(sample_num: i32) -> RandomSampler {
        RandomSampler { sample_num }
    }
}

impl Sampler for RandomSampler {
    fn sample(&self) -> Vec<Coord2D> {
        let mut ret = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..self.sample_num {
            let x = rng.gen::<f64>();
            let y = rng.gen::<f64>();
            ret.push(Coord2D::new(x, y));
        }
        ret
    }

    fn sample_num(&self) -> i32 {
        self.sample_num
    }
}