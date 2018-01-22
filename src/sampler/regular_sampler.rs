use super::*;
use utility::*;

pub struct RegularSampler {
    row: i32,
    col: i32
}

impl RegularSampler {
    pub fn new(row: i32, col: i32) -> RegularSampler {
        RegularSampler { row, col }
    }
}

impl Sampler for RegularSampler {
    fn sample(&self) -> Vec<Coord2D> {
        let mut ret = vec![];
        let row_step = 1. / self.row as f64;
        let col_step = 1. / self.col as f64;
        let row_start = row_step / 2.;
        let col_start = col_step / 2.;

        for i in 0..self.row {
            for j in 0..self.col {
                ret.push(Coord2D::new(row_start + i as f64 * row_step, col_start + j as f64 * col_step));
            }
        }
        ret
    }

    fn sample_num(&self) -> i32 {
        self.row * self.col
    }
}