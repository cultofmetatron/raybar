
use std::cmp::{Eq, PartialEq};

fn main() {
    println!("Hello, world!");
}

/*
    GlPrimative is a shared set of traints between the point, vector and matrix operations 
    allowing all 3 to interoperate with each other
*/

// for comparing floating points
enum FlCompare {
    GT,
    LT,
    EQ
}

fn fl_cmp(a: f64, b:f64) -> FlCompare {
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

fn fl_eq(a: f64, b: f64) -> bool {
    match fl_cmp(a, b) {
        FlCompare::EQ => true,
        _ => false
    }
}


#[allow(dead_code)]
trait GlPrimative {
    fn to_tuple(self) -> (f64, f64, f64, usize);
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct GlPoint {
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
    fn new(x: f64, y: f64, z: f64) -> GlPoint {
        GlPoint {
            x,
            y,
            z
        }
    }
}

impl GlPrimative for GlPoint {
    fn to_tuple(self) -> (f64, f64, f64, usize) {
        (self.x, self.y, self.z, 0)
    } 
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
struct GlVector {
    x: f64,
    y: f64,
    z: f64,
}

impl GlVector {
    #[allow(dead_code)]
    fn new(x: f64, y: f64, z: f64) -> GlVector {
        GlVector {
            x,
            y,
            z
        }
    }
}

impl PartialEq for GlVector {
    fn eq(&self, b: &GlVector) -> bool {
        fl_eq(self.x, b.x) && fl_eq(self.y, b.y) && fl_eq(self.z, b.z)
    }
}

impl GlPrimative for GlVector {
    fn to_tuple(self) -> (f64, f64, f64, usize) {
        (self.x, self.y, self.z, 1)
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
    fn test_add() {
        assert_eq!(GlPoint::new(2.3, 42.5, 84.0).to_tuple(), (2.3, 42.5, 84.0, 0));
    }


    /* Scenario 
        Scenario: GlVector() creates tuples with w=0
            Given v ← GlVector(4, -4, 3)
            Then v = GlVector(4, -4, 3)
    */
    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(1 + 1 + 1, 3);
    }
}