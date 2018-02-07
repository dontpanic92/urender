use super::*;
use utility::*;
use rand;
use rand::Rng;

pub struct NRooksSampler {
    sample_num: i32
}

impl NRooksSampler {
    pub fn new(sample_num: i32) -> NRooksSampler {
        NRooksSampler { sample_num }
    }
}

impl Sampler for NRooksSampler {
    fn sample(&self) -> Vec<Coord2D> {
        let mut x_list = vec![];
        let mut y_list = vec![];
        let step = 1. / self.sample_num as f64;
        let mut rng = rand::thread_rng();

        for i in 0..self.sample_num {
            let x = (rng.gen::<f64>() + i as f64) * step;
            let y = (rng.gen::<f64>() + i as f64) * step;
            x_list.push(x);
            y_list.push(y);
        }

        rng.shuffle(&mut x_list);
        rng.shuffle(&mut y_list);

        x_list.iter().zip(y_list.iter()).map(|(x, y)| Coord2D::new(*x, *y)).collect()
    }

    fn sample_num(&self) -> i32 {
        self.sample_num
    }
}