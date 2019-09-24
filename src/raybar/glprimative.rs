

//use std::cmp::{Ord, PartialEq, PartialOrd};
//use std::ops::{Add, Div, Mul, Sub};
//use num_traits::{One, Signed, ToPrimitive, Zero};
/*
    GlPrimative is a shared set of traints between the point, vector and matrix operations
    allowing all 3 to interoperate with each other
*/
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
//std::clone::Clone;
//std::copy::{Copy};
use std::ops::{Add, Div, Mul, Sub};
//use std::f64::consts::PI;
//use std::num::{FpCategory};
use num_traits::{One, Signed, ToPrimitive, Zero, Float};

// for comparing floating points
//use super::util::fl_eq;


pub trait MatrixNumber = PartialEq
            + PartialOrd
            + Mul<Output = Self>
            + Add<Output = Self>
            + Sub<Output = Self>
            + Div<Output = Self>
            + Clone
            + Copy
            + Zero
            + One
            + Signed
            + ToPrimitive
            + Float
            + Debug;

#[allow(dead_code)]
pub trait GlPrimative<T: MatrixNumber, G> {
    type Output;
    fn dot(&self, rhs: &G) -> Self::Output;
}

pub trait Cross<T: MatrixNumber, G> {
    type Output;
    type Input;
    fn cross(&self, rhs: &Self::Input) -> Self::Output;
}

