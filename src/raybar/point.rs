use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero, Float};

use super::glprimative::GlPrimative;
use super::matrix::{GlMatrix, MatrixNumber};

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
}