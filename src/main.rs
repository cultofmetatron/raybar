fn main() {
    println!("Hello, world!");
}

/*
    GlPrimative is a shared set of traints between the point, vector and matrix operations 
    allowing all 3 to interoperate with each other
*/
#[allow(dead_code)]
trait GlPrimative {

}

#[allow(dead_code)]
struct GlPoint {
    x: f64,
    y: f64,
    z: f64,
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

#[allow(dead_code)]
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


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(1 + 1 + 1, 3);
    }
}