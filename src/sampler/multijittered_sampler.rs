use super::*;
use utility::*;
use rand;
use rand::Rng;

pub struct MultiJitteredSampler {
    row: i32,
    col: i32
}

impl MultiJitteredSampler {
    pub fn new(row: i32, col: i32) -> MultiJitteredSampler {
        MultiJitteredSampler { row, col }
    }
}

impl Sampler for MultiJitteredSampler {
    fn sample(&self) -> Vec<Coord2D> {
        let mut ret = vec![];
        let row_step = 1. / self.row as f64;
        let col_step = 1. / self.col as f64;
        let mut rng = rand::thread_rng();

        // initial arrangement
        for j in 0..self.row {
            for i in 0..self.col {
                let x = (i as f64 + (j as f64 + rng.gen::<f64>()) * row_step) * col_step;
                let y = (j as f64 + (i as f64 + rng.gen::<f64>()) * col_step) * row_step;
                ret.push(Coord2D::new(x, y));
            }
        }

        // correlated shuffle
        for j in 0..self.row {
            let k = j + (rng.gen::<f64>() * (self.row - j) as f64) as i32;
            for i in 0..self.col {
                let index1 = (j * self.col + i) as usize;
                let index2 = (k * self.col + i) as usize;
                let coord1 = ret[index1];
                let coord2 = ret[index2];
                ret[index1] = Coord2D::new(coord2.x(), coord1.y());
                ret[index2] = Coord2D::new(coord1.x(), coord2.y());
            }
        }

        for i in 0..self.col {
            let k = i + (rng.gen::<f64>() * (self.col - i) as f64) as i32;
            for j in 0..self.row {
                let index1 = (j * self.col + i) as usize;
                let index2 = (j * self.col + k) as usize;
                let coord1 = ret[index1];
                let coord2 = ret[index2];
                ret[index1] = Coord2D::new(coord1.x(), coord2.y());
                ret[index2] = Coord2D::new(coord2.x(), coord1.y());
            }
        }

        ret
    }

    fn sample_num(&self) -> i32 {
        self.row * self.col
    }
}