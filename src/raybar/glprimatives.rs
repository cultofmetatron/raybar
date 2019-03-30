

use std::cmp::{PartialEq};
use std::ops;
/*
    GlPrimative is a shared set of traints between the point, vector and matrix operations 
    allowing all 3 to interoperate with each other
*/

// for comparing floating points
pub enum FlCompare {
    GT,
    LT,
    EQ
}

pub fn fl_cmp(a: f64, b:f64) -> FlCompare {
    let epsilon: f64 = 0.0001;
    let diff = a - b;
    if diff.abs() < epsilon {
        return FlCompare::EQ;
    } else if diff > 0.0 {
        return FlCompare::GT;
    } else {
        return FlCompare::LT; 
    }
}

pub fn fl_eq(a: f64, b: f64) -> bool {
    match fl_cmp(a, b) {
        FlCompare::EQ => true,
        _ => false
    }
}



#[allow(dead_code)]
pub trait GlPrimative {
    fn to_tuple(self) -> (f64, f64, f64, usize);
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct GlPoint {
    x: f64,
    y: f64,
    z: f64,
}

impl PartialEq for GlPoint {
    fn eq(&self, b: &GlPoint) -> bool {
      fl_eq(self.x, b.x) && fl_eq(self.y, b.y) && fl_eq(self.z, b.z)
    }
}

impl GlPoint {
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> GlPoint {
        GlPoint {
            x,
            y,
            z
        }
    }
}

impl GlPrimative for GlPoint {
    fn to_tuple(self) -> (f64, f64, f64, usize) {
        (self.x, self.y, self.z, 1)
    } 
}

impl ops::Add<GlVector> for GlPoint {
    type Output = GlPoint;

    fn add(self, rhs: GlVector) -> GlPoint {
        GlPoint::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}


impl ops::Sub<GlPoint> for GlPoint {
    type Output = GlVector;

    fn sub(self, rhs: GlPoint) -> GlVector {
        GlVector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Sub<GlVector> for GlPoint {
    type Output = GlPoint;
    fn sub(self, rhs: GlVector) -> GlPoint {
        GlPoint::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}


#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct GlVector {
    x: f64,
    y: f64,
    z: f64,
}

impl GlVector {
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> GlVector {
        GlVector {
            x,
            y,
            z
        }
    }
    #[allow(dead_code)]
    pub fn negate(self) -> GlVector {
        let zero_vector = GlVector::new(0.0, 0.0, 0.0);
        zero_vector - self
    }
    #[allow(dead_code)]
    pub fn magnitude(self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }
    #[allow(dead_code)]
    pub fn normalize(self) -> GlVector {
        let mag: f64 = self.magnitude();
        GlVector::new(
            self.x / mag,
            self.y / mag,
            self.z / mag
        )
    }
    #[allow(dead_code)]
    pub fn dot(self, v2: GlVector) -> f64 {
        self.x * v2.x + self.y * v2.y + self.z * v2.z
    }
    #[allow(dead_code)]
    pub fn cross(self, v2: GlVector) -> GlVector {
        GlVector::new(
            self.y * v2.z - self.z * v2.y,
            self.z * v2.x - self.x * v2.z,
            self.x * v2.y - self.y * v2.x
        )
    }
}

impl PartialEq for GlVector {
    fn eq(&self, b: &GlVector) -> bool {
        fl_eq(self.x, b.x) && fl_eq(self.y, b.y) && fl_eq(self.z, b.z)
    }
}

impl GlPrimative for GlVector {
    fn to_tuple(self) -> (f64, f64, f64, usize) {
        (self.x, self.y, self.z, 0)
    } 
}

impl ops::Add<GlVector> for GlVector {
    type Output = GlVector;
    fn add(self, rhs: GlVector) -> GlVector {
        GlVector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub<GlVector> for GlVector {
    type Output = GlVector;
    
    fn sub(self, rhs: GlVector) -> GlVector {
        GlVector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Neg for GlVector {
    type Output = GlVector;
    fn neg(self) -> GlVector {
        self.negate()
    }
}

impl ops::Mul<f64> for GlVector {
    type Output = GlVector;
    fn mul(self, rhs: f64) -> GlVector {
        GlVector::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Div<f64> for GlVector {
    type Output = GlVector;
    fn div(self, rhs: f64) -> GlVector {
        GlVector::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}
