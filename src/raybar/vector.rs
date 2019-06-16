use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub, Neg};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero};

use super::glprimative::{MatrixNumber, GlPrimative};
use super::matrix::{GlMatrix};
use super::point::GlPoint;


#[allow(dead_code)]
#[derive(Clone, Debug)]
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
    &self.matrix.get(0, 1)
  }
  pub fn get_z(&self) -> &T {
    &self.matrix.get(0, 2)
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
    pub fn test_create_vector() {

    }

    pub fn test_vector_plus_vector() {

    }

}