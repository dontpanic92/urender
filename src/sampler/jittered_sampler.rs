use super::*;
use utility::*;
use rand;
use rand::Rng;

pub struct JitteredSampler {
    row: i32,
    col: i32
}

impl JitteredSampler {
    pub fn new(row: i32, col: i32) -> JitteredSampler {
        JitteredSampler { row, col }
    }
}

impl Sampler for JitteredSampler {
    fn sample(&self) -> Vec<Coord2D> {
        let mut ret = vec![];
        let row_step = 1. / self.row as f64;
        let col_step = 1. / self.col as f64;
        let mut rng = rand::thread_rng();

        for i in 0..self.col {
            for j in 0..self.row {
                let x = rng.gen::<f64>() * col_step;
                let y = rng.gen::<f64>() * row_step;
                ret.push(Coord2D::new(i as f64 * col_step + x, j as f64 * row_step + y));
            }
        }
        ret
    }

    fn sample_num(&self) -> i32 {
        self.row * self.col
    }
}