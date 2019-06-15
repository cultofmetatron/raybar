extern crate num_traits;

use std::cmp::{Ord, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Sub};
//use num_traits::{One, Signed, ToPrimitive, Zero};
/*
    GlPrimative is a shared set of traints between the point, vector and matrix operations
    allowing all 3 to interoperate with each other
*/

// for comparing floating points
use super::util::fl_eq;

#[allow(dead_code)]
pub trait GlPrimative {
    type Output;
    type Input;
    fn dot(&self, rhs: &Self::Input) -> Self::Output;
}
