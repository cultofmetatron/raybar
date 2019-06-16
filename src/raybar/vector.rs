use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
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