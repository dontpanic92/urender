use std::fmt::{self, Formatter, Display};


#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Coord2D {
    x: f64,
    y: f64
}

impl Coord2D {
    pub fn new<Tx: Into<f64>, Ty: Into<f64>>(x: Tx, y: Ty) -> Coord2D {
        Coord2D { x: x.into(), y: y.into() }
    }
    
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
}

impl Display for Coord2D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}