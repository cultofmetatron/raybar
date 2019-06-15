use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero, Float};

use super::glprimative::GlPrimative;
use super::matrix::GlMatrix;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GlPoint<T: PartialEq
            + PartialOrd
            + Mul<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Clone
            + Copy
            + Zero
            + One
            + Signed
            + ToPrimitive
            + Float
            + Debug> {
  matrix: GlMatrix<T>
}

impl<T: PartialEq
            + PartialOrd
            + Mul<Output = T>
            + Add<Output = T>
            + Sub<Output = T>
            + Div<Output = T>
            + Clone
            + Copy
            + Zero
            + One
            + Signed
            + ToPrimitive
            + Float
            + Debug> GlPoint<T> {

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