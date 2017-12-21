use super::*;
use std::ops::{Add, Sub};
use std::fmt::{self, Formatter, Display};


#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Coord3D {
    x: f64,
    y: f64,
    z: f64
}

impl Coord3D {
    pub fn new<Tx: Into<f64>, Ty: Into<f64>, Tz: Into<f64>>(x: Tx, y: Ty, z: Tz) -> Coord3D {
        Coord3D { x: x.into(), y: y.into(), z: z.into() }
    }
    
    pub fn x(&self) -> f64 {
        self.x
    }
    
    pub fn y(&self) -> f64 {
        self.y
    }
    
    pub fn z(&self) -> f64 {
        self.z
    }
}

impl Display for Coord3D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Vector3D> for Coord3D {
    type Output = Coord3D;
    
    fn add(self, rhs: Vector3D) -> Coord3D {
        Coord3D::new(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
}

impl Sub for Coord3D {
    type Output = Vector3D;
    
    fn sub(self, rhs: Coord3D) -> Vector3D {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}