use std::ops::{Add, Sub, Mul, Neg};
use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
pub struct Vector3D {
    x: f64,
    y: f64,
    z: f64
}

impl Vector3D {
    pub fn new<Tx: Into<f64>, Ty: Into<f64>, Tz: Into<f64>>(x: Tx, y: Ty, z: Tz) -> Vector3D {
        Vector3D { x: x.into(), y: y.into(), z: z.into() }
    }
    
    pub fn dot(&self, rhs: Vector3D) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    
    pub fn cross(&self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(self.y * rhs.z - self.z * rhs.y, self.z * rhs.x - self.x * rhs.z, self.x * rhs.y -self.y * rhs.x)
    }

    pub fn normal(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let normal = self.normal();
        Vector3D::new(self.x / normal, self.y / normal, self.z / normal)
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

impl Display for Vector3D {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for Vector3D {
    type Output = Vector3D;
    
    fn add(self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;
    
    fn sub(self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Into<f64>> Mul<T> for Vector3D {
    type Output = Vector3D;
    
    fn mul(self, rhs: T) -> Vector3D {
        let t = rhs.into();
        Vector3D::new(t * self.x, t * self.y, t * self.z)
    }
}

impl Neg for Vector3D {
    type Output = Vector3D;

    fn neg(self) -> Vector3D {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<Vector3D> for i32 {
    type Output = Vector3D;
    
    fn mul(self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(self as f64 * rhs.x, self as f64 * rhs.y, self as f64 * rhs.z)
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;
    
    fn mul(self, rhs: Vector3D) -> Vector3D {
        Vector3D::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}
