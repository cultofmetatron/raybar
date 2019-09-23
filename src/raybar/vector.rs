use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Sub, Neg};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero, Float};

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
  pub fn from_matrix(content: GlMatrix<T>) -> GlVector<T> {
    if content.get_col_size() == 4 && content.get_row_size() == 1 && *content.get(3, 0) == Zero::zero() {
      GlVector{
        matrix: content
      }      
    } else {
      panic!("invalid input matrix for vector!");
    }
  }
  pub fn get_matrix(&self) -> &GlMatrix<T> {
    &self.get_matrix()
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
  #[allow(dead_code)]
  pub fn magnitude(&self) -> T {
    Float::sqrt(
      *self.get_x() * *self.get_x() +
      *self.get_y() * *self.get_y() +
      *self.get_z() * *self.get_z()
    )
  }
  #[allow(dead_code)]
  pub fn normalize(&self) -> GlVector<T> {
    let mag = self.magnitude();
    GlVector::new(*self.get_x()/ mag, *self.get_y() / mag, *self.get_z() / mag)
  }
  #[allow(dead_code)]
  pub fn dot(&self, v2: &GlVector<T>) -> T {
      self.get_x().clone() * v2.get_x().clone() +
      self.get_y().clone() * v2.get_y().clone() +
      self.get_z().clone() * v2.get_z().clone()
  }
  pub fn cross(&self, v2: &GlVector<T>) -> GlVector<T> {
    GlVector::new(
            *self.get_y() * *v2.get_z() - *self.get_z() * *v2.get_y(),
            *self.get_z() * *v2.get_x() - *self.get_x() * *v2.get_z(),
            *self.get_x() * *v2.get_y() - *self.get_y() * *v2.get_x(),
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


impl<T: MatrixNumber> Mul<T> for GlVector<T> {
    type Output = GlVector<T>;
    fn mul(self, rhs: T) -> GlVector<T> {
        GlVector::new(
          *self.get_x() * rhs,
          *self.get_y() * rhs,
          *self.get_z() * rhs)
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

    #[test]
    fn test_vector_minus_vector() {
        let a = GlVector::new(2.3, 42.5, 84.0);
        let b = GlVector::new(2.3, 42.5, 84.0);
        let new_vector = a - b;
        assert_eq!(new_vector, GlVector::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_vector_negation() {
        assert_eq!(
            GlVector::new(4.0, -2.0, -8.0).negate(),
            GlVector::new(-4.0, 2.0, 8.0)
        );
    }

    #[test]
    fn test_vector_mult_scalar() {
        assert_eq!(
            GlVector::new(2.0, 4.0, 6.0) * 2.0,
            GlVector::new(4.0, 8.0, 12.0)
        );
    }

    #[test]
    fn test_vector_magnitude() {
        assert_eq!(GlVector::new(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(GlVector::new(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(GlVector::new(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(GlVector::new(0.0, 1.0, 1.0).magnitude(), Float::sqrt(2.0));
    }
    #[test]
    fn test_vector_normalize() {
        assert_eq!(
            GlVector::new(4.0, 0.0, 0.0).normalize(),
            GlVector::new(1.0, 0.0, 0.0)
        );
        assert_eq!(
            GlVector::new(0.0, 6.0, 0.0).normalize(),
            GlVector::new(0.0, 1.0, 0.0)
        );
        assert_eq!(
            GlVector::new(1.0, 2.0, 3.0).normalize(),
            GlVector::new(0.2672612419124244, 0.5345224838248488, 0.8017837257372732)
        );
    }

    #[test]
    fn test_vector_dot_product() {
        let a = GlVector::new(1.0, 2.0, 3.0);
        let b = GlVector::new(2.0, 3.0, 4.0);
        assert_eq!(a.dot(&b), 20.0);
    }

    #[test]
    fn test_vector_cross_product() {
        let a = GlVector::new(1.0, 2.0, 3.0);
        let b = GlVector::new(2.0, 3.0, 4.0);
        assert_eq!(a.cross(&b), GlVector::new(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), GlVector::new(1.0, -2.0, 1.0));
    }

}