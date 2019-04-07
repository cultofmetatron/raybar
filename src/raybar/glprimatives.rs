

use std::cmp::{PartialEq};
use std::ops;
/*
    GlPrimative is a shared set of traints between the point, vector and matrix operations 
    allowing all 3 to interoperate with each other
*/

// for comparing floating points
use super::util::{fl_eq};



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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /* Scenario 
        Scenario: GlPoint() creates point
            Given p ← GlPoint::new(4, -4, 3)
            Then p = GlPoint{4, -4, 3}
    */
    #[test]
    fn test_create_point() {
        assert_eq!(GlPoint::new(2.3, 42.5, 84.0).to_tuple(), (2.3, 42.5, 84.0, 1));
    }


    /* Scenario 
        Scenario: GlVector() creates tuples with w=0
            Given v ← GlVector(4, -4, 3)
            Then v = GlVector(4, -4, 3)
    */
    #[test]
    fn test_create_vector() {
        assert_eq!(GlVector::new(2.3, 42.5, 84.0).to_tuple(), (2.3, 42.5, 84.0, 0));
    }

    #[test]
    fn test_point_plus_vector() {
        let a: GlPoint = GlPoint::new(2.3, 42.5, 84.0);
        let b: GlVector = GlVector::new(2.3, 42.5, 84.0);
        let new_point: GlPoint = a + b;
        assert_eq!(new_point, GlPoint::new(4.6, 85.0, 168.0));
    }

    #[test]
    fn test_vector_plus_vector() {
        let a: GlVector = GlVector::new(2.3, 42.5, 84.0);
        let b: GlVector = GlVector::new(2.3, -42.5, 84.0);
        let new_vector: GlVector = a + b;
        assert_eq!(new_vector, GlVector::new(4.6, 0.0, 168.0));
    }

    #[test]
    fn test_point_minus_point() {
        let a: GlPoint = GlPoint::new(2.3, 42.5, 84.0);
        let b: GlPoint = GlPoint::new(2.3, 42.5, 0.0);
        let new_vector = a - b;
        assert_eq!(new_vector, GlVector::new(0.0, 0.0, 84.0));
    }
    #[test]
    fn test_point_minus_vector() {
        let a: GlPoint = GlPoint::new(2.3, 42.5, 84.0);
        let b: GlVector = GlVector::new(2.3, 42.5, 0.0);
        let new_point = a - b;
        assert_eq!(new_point, GlPoint::new(0.0, 0.0, 84.0));
    }
    #[test]
    fn test_vector_minus_vector() {
        let a: GlVector = GlVector::new(2.3, 42.5, 84.0);
        let b: GlVector = GlVector::new(2.3, 42.5, 84.0);
        let new_vector: GlVector = a - b;
        assert_eq!(new_vector, GlVector::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vector_negation() {
        assert_eq!(GlVector::new(4.0, -2.0, -8.0).negate(), GlVector::new(-4.0, 2.0, 8.0));
    } 
    #[test]
    fn test_vector_unary_negation() {
        assert_eq!(-GlVector::new(4.0, -2.0, -8.0), GlVector::new(-4.0, 2.0, 8.0));
    }

    #[test]
    fn test_vector_mult_scalar() {
        assert_eq!(GlVector::new(2.0, 4.0, 6.0) * 2.0, GlVector::new(4.0, 8.0, 12.0));
    }
    #[test]
    fn test_vector_div_scalar() {
        assert_eq!(GlVector::new(2.0, 4.0, 6.0) / 2.0, GlVector::new(1.0, 2.0, 3.0));
    }
    #[test]
    fn test_vector_magnitude() {
        assert_eq!(GlVector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(GlVector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(GlVector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(GlVector::new(0.0, 1.0, 1.0).magnitude(), f64::sqrt(2.0));
    }

    #[test]
    fn test_vector_normalize() {
        assert_eq!(GlVector::new(4.0, 0.0, 0.0).normalize(), GlVector::new(1.0, 0.0, 0.0));
        assert_eq!(GlVector::new(0.0, 6.0, 0.0).normalize(), GlVector::new(0.0, 1.0, 0.0));
        assert_eq!(GlVector::new(1.0, 2.0, 3.0).normalize(), GlVector::new(0.26726, 0.53452, 0.80178));
    }

    #[test]
    fn test_vector_dot_product() {
        let a: GlVector = GlVector::new(1.0, 2.0, 3.0);
        let b: GlVector = GlVector::new(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let a: GlVector = GlVector::new(1.0, 2.0, 3.0);
        let b: GlVector = GlVector::new(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), GlVector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), GlVector::new(1.0, -2.0, 1.0));
    }
}