use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero};

use super::glprimative::{MatrixNumber, GlPrimative};
use super::matrix::{GlMatrix};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GlPoint<T: MatrixNumber> {
  matrix: GlMatrix<T>
}

impl<T: MatrixNumber> GlPoint<T> {

  #[allow(dead_code)]
  pub fn new(x: T, y: T, z: T) -> GlPoint<T> {

    GlPoint{
      matrix: GlMatrix::new(vec![
          vec![x],
          vec![y],
          vec![z],
          vec![One::one()]
      ])
    }
  }
  #[allow(dead_code)]
  fn get_x(&self) -> &T {
    self.matrix.get(0, 0)
  }
  fn get_y(&self) -> &T {
    self.matrix.get(1, 0)
  }
  fn get_z(&self) -> &T {
    self.matrix.get(2, 0)
  }
  #[allow(dead_code)]
  pub fn to_tuple(&self) -> (T, T, T, T) {
    //let col = self.matrix.get_column(0);
    (
      self.get_x().clone(),
      self.get_y().clone(),
      self.get_z().clone(),
      One::one()
    )
  }
}

impl<T: MatrixNumber> PartialEq for GlPoint<T> {
    fn eq(&self, b: &GlPoint<T>) -> bool {
        self.matrix == b.matrix
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
        assert_eq!(
            GlPoint::new(2.3, 42.5, 84.0).to_tuple(),
            (2.3, 42.5, 84.0, 1.0)
        );
    }

}