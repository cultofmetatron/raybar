use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub, Neg};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero};

use super::glprimative::{MatrixNumber, GlPrimative};
use super::matrix::{GlMatrix};
use super::point::GlPoint;


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct GlVector<T: MatrixNumber> {
  matrix: GlMatrix<T>
}

impl<T: MatrixNumber> GlVector<T> {
    #[allow(dead_code)]
  pub fn new(x: T, y: T, z: T) -> GlVector<T> {
    GlVector{
      matrix: GlMatrix::new(vec![
          vec![x],
          vec![y],
          vec![z],
          vec![Zero::zero()]
      ])
    }
  }
  #[allow(dead_code)]
  pub fn negate(&self) -> GlVector<T> {
      let zero_vector = GlVector::new(Zero::zero(), Zero::zero(), Zero::zero());
      zero_vector - self.clone()
  }
  pub fn get_x(&self) -> &T {
    &self.matrix.get(0, 0)
  }
  pub fn get_y(&self) -> &T {
    &self.matrix.get(1, 0)
  }
  pub fn get_z(&self) -> &T {
    &self.matrix.get(2, 0)
  }
  #[allow(dead_code)]
  pub fn to_tuple(&self) -> (T, T, T, T) {
    (
      (self.get_x()).clone(), 
      (self.get_y()).clone(),
      (self.get_z()).clone(),
      Zero::zero()
    )
  }
}

impl<T: MatrixNumber> Add<GlVector<T>> for GlVector<T> {
    type Output = GlVector<T>;
    fn add(self, rhs: GlVector<T>) -> GlVector<T> {
        GlVector::new(
          *self.get_x() + *rhs.get_x(),
          *self.get_y() + *rhs.get_y(),
          *self.get_z() + *rhs.get_z()
        )
    }
}

impl<T: MatrixNumber> Sub<GlVector<T>> for GlVector<T> {
    type Output = GlVector<T>;
    fn sub(self, rhs: GlVector<T>) -> GlVector<T> {
        GlVector::new(
          *self.get_x() - *rhs.get_x(),
          *self.get_y() - *rhs.get_y(),
          *self.get_z() - *rhs.get_z()
        )
    }
}

impl<T: MatrixNumber> Neg for GlVector<T> {
    type Output = GlVector<T>;
    fn neg(self) -> GlVector<T> {
        self.negate()
    }
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    #[test]
    pub fn test_create_vector() {
       assert_eq!(
            GlVector::new(2.3, 42.5, 84.0).to_tuple(),
            (2.3, 42.5, 84.0, 0.0)
        );
    }
    #[test]
    pub fn test_vector_plus_vector() {
      let a: GlVector<f64> = GlVector::new(2.3, 42.5, 84.0);
      let b: GlVector<f64> = GlVector::new(2.3, -42.5, 84.0);
      let new_vector = a + b;
      assert_eq!(new_vector, GlVector::new(4.6, 0.0, 168.0));
    }

}