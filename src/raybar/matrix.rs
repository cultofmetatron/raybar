/*
  matrix primatives for raybar

  todo
  create NxM matrix
  where N -> row
  where M -> col
  where for matrix A, A[n][m] yields the item at row n, col m

*/
use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;
//std::clone::Clone;
//std::copy::{Copy};
use std::ops::{Add, Div, Mul, Sub};
//use std::f64::consts::PI;
//use std::num::{FpCategory};
extern crate num_traits;
use num_traits::{One, Signed, ToPrimitive, Zero};

use super::glprimative::{GlPrimative};


#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct GlMatrix<T: PartialEq + PartialOrd> {
    content: Vec<Vec<T>>,
}

impl<
        T: PartialEq
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
            + Debug,
    > GlMatrix<T>
{
    #[allow(dead_code)]
    pub fn new(content: Vec<Vec<T>>) -> GlMatrix<T> {
        // for each sup vector, the length must be consitent
        if GlMatrix::vec_sizing(&content) {
            GlMatrix { content: content }
        } else {
            panic!("all rows must be the same size!");
        }
    }
    /*
        generates an identity matrix for a given piece of code
    */
    #[allow(dead_code)]
    pub fn identity(size: usize) -> GlMatrix<T> {
        let mut rows = vec![];
        for i in 0..size {
            let mut row: Vec<T> = vec![];
            for j in 0..size {
                if j == i {
                    row.push(One::one());
                } else {
                    row.push(Zero::zero());
                }
            }
            rows.push(row);
        }
        GlMatrix::new(rows)
    }
    #[allow(dead_code)]
    pub fn translation(x: T, y: T, z: T) -> GlMatrix<T> {
        GlMatrix::new(vec![
            vec![One::one(), Zero::zero(), Zero::zero(), x],
            vec![Zero::zero(), One::one(), Zero::zero(), y],
            vec![Zero::zero(), Zero::zero(), One::one(), z],
            vec![Zero::zero(), Zero::zero(), Zero::zero(), One::one()]
        ])
    }
    #[allow(dead_code)]
    pub fn transpose(&self) -> GlMatrix<T> {
        let mut rows = vec![];
        for j in 0..self.get_col_size() {
            let column = self.get_column(j);
            rows.push(column);
        }
        GlMatrix::new(rows)
    }
    #[allow(dead_code)]
    pub fn get(&self, n: usize, m: usize) -> &T {
        &self.content[n][m]
    }
    #[allow(dead_code)]
    pub fn set(&mut self, n: usize, m: usize, value: T) {
        self.content[n][m] = value;
    }
    #[allow(dead_code)]
    pub fn get_row_size(&self) -> usize {
        self.content.len()
    }
    #[allow(dead_code)]
    pub fn get_col_size(&self) -> usize {
        self.content[0].len()
    }
    #[allow(dead_code)]
    pub fn get_dimensions(&self) -> (usize, usize) {
        (self.get_row_size(), self.get_col_size())
    }
    #[allow(dead_code)]
    pub fn is_square(&self) -> bool {
        let (m, n) = self.get_dimensions();
        m == n
    }
    

    #[allow(dead_code)]
    pub fn get_row(&self, index: usize) -> Vec<T> {
        self.content[index].clone()
    }
    #[allow(dead_code)]
    pub fn get_column(&self, index: usize) -> Vec<T> {
        // for each row, get the ith value and push it into the vector
        let mut column: Vec<T> = vec![];
        for row in self.content.clone().into_iter() {
            column.push(row[index].clone());
        }
        column
    }
    /*
        removes column and row from the matrix and returns a new smaller matrix
    */
    #[allow(dead_code)]
    pub fn submatrix(&self, row: usize, col: usize) -> GlMatrix<T> {
        let new_contents: Vec<Vec<T>> = self
            .content
            .clone()
            .iter()
            .enumerate()
            .filter(|&(row_i, _column)| row_i != row)
            .map(|(_i, column)| {
                // remve the value at col
                column
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != col)
                    .map(|(_j, col)| col.clone())
                    .collect()
            })
            .collect();
        GlMatrix::new(new_contents)
    }
    #[allow(dead_code)]
    pub fn cofactor(&self, row: usize, col: usize) -> T {
        let minor: T = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
    #[allow(dead_code)]
    pub fn minor(&self, row: usize, col: usize) -> T {
        self.submatrix(row, col).det()
    }
    pub fn det(&self) -> T {
        if self.is_square() {
            self.determinate()
        } else {
            panic!("non square matrices have no determinant!");
        }
    }
    #[allow(dead_code)]
    pub fn is_invertible(&self) -> bool {
        self.det() != Zero::zero()
    }
    #[allow(dead_code)]
    pub fn invert(&self) -> Option<GlMatrix<f64>> {
        let size = self.get_col_size();
        let det = self.det(); //determing the deterinate
                              // a zero determineate indicates there is no inverse to be had
        if self.is_square() {
            Option::None
        } else if det == Zero::zero() {
            Option::None
        } else {
            let mut invert_contents: Vec<Vec<f64>> = (0..size)
                .map(|_row| (0..size).map(|_val| 0.0).collect())
                .collect();
            let floating_det = det.to_f64().unwrap();
            for row in 0..size {
                for col in 0..size {
                    invert_contents[col][row] = self
                        .cofactor(row, col)
                        .to_f64()
                        .map(|cofactor| cofactor / floating_det)
                        .unwrap()
                }
            }

            Option::Some(GlMatrix::new(invert_contents))
        }
    }
    #[allow(dead_code)]
    pub fn mult(&self, scaler: T) -> GlMatrix<T> {
        let contents = self
            .content
            .iter()
            .map(|row| row.iter().map(|val| *val * scaler).collect())
            .collect();
        GlMatrix::new(contents)
    }
    /*
        dumps the data structre into an array of f64s for the prupose of raytracing
    */
    #[allow(dead_code)]
    pub fn to_floats(&self) -> GlMatrix<f64> {
        let contents = self
            .content
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| (*val).to_f64().unwrap_or(0.0))
                    .collect()
            })
            .collect();
        GlMatrix::new(contents)
    }
        #[allow(dead_code)]
    fn dot_list(row: &Vec<T>, col: &Vec<T>) -> Option<T> {
        if row.len() == 0 {
            return None; //hack for dealing with null generics
        }
        if row.len() == col.len() {
            let mut i = 0;
            let mut acc = row[i].clone() * col[i].clone();
            loop {
                i = i + 1;
                if i == row.len() {
                    break;
                } else {
                    acc = acc + (row[i].clone() * col[i].clone());
                }
            }
            return Option::Some(acc);
        } else {
            panic!("Invalid Matrix ");
        }
    }

    #[allow(dead_code)]
    fn determinate(&self) -> T {
        if self.get_row_size() == 2 {
            self.det_base()
        } else {
            //panic!("cannpt comput greater than 2 yet");
            let size = self.get_col_size();
            (0..size)
                .into_iter()
                .map(|i| self.cofactor(0, i) * *self.get(0, i))
                .fold(Zero::zero(), |acc, cofactor| acc + cofactor)
        }
    }
    #[allow(dead_code)]
    fn det_base(&self) -> T {
        //for the case where the matrix is 2 x 2
        let a = &self.content[0][0];
        let d = &self.content[1][1];
        let b = &self.content[0][1];
        let c = &self.content[1][0];
        // we deref them here for borrowing
        let ret = (*a * *d) - (*b * *c);
        println!("in det_base {:?}*{:?} - {:?}*{:?} {:?}", a, d, b, c, ret);
        ret
    }
    #[allow(dead_code)]
    fn vec_sizing(list: &Vec<Vec<T>>) -> bool {
        /*
          for a list of sub lists, if the list's length is 0, return false
          if the list is one, return true
          if the list is greater than that,
            for each list in list, store the size, if it changes, return false
        */
        match list.len() {
            0 => false,
            1 => {
                if list[0].len() == 0 {
                    false
                } else {
                    true
                }
            }
            _ => {
                let acc = list[0].len();
                for sub_list in list[1..list.len()].into_iter() {
                    if acc != sub_list.len() {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

impl<
        T: PartialEq
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
            + Debug,
    > GlPrimative for GlMatrix<T> {
    type Output = GlMatrix<T>;
    type Input = GlMatrix<T>;

    #[allow(dead_code)]
    fn dot(&self, b: &Self::Input) -> GlMatrix<T> {
        let mut contents: Vec<Vec<T>> = vec![];
        //for each row, compute all the values
        let row_size = self.get_row_size();
        let col_size = b.get_col_size();
        // row size must equal col size or we should panic as it is an invalid operation
        if row_size != col_size {
            panic!("invalid matrix operation: invalid dimensions for dot product");
        } else {
            let mut i = 0;
            loop {
                //iterate through each row and grab the jth col
                if i == row_size {
                    break;
                } else {
                    let row: Vec<T> = self.get_row(i);
                    let mut new_row: Vec<T> = vec![];
                    let mut j = 0;
                    loop {
                        if j == col_size {
                            break;
                        } else {
                            let column = b.get_column(j);
                            let dot_product_i_j =
                                GlMatrix::dot_list(&row, &column).unwrap_or(Zero::zero());
                            new_row.push(dot_product_i_j);
                            j = j + 1;
                        }
                    }
                    contents.push(new_row);
                    i = i + 1;
                }
            }
            return GlMatrix::new(contents);
        }
    }


}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_matrix_new() {
        // can create new test
        let matrix: GlMatrix<i64> =
            GlMatrix::new(vec![vec![1, 2, 3], vec![6, 2, 7], vec![21, 45, 3]]);
        assert_eq!(matrix.get_dimensions(), (3, 3));
        assert!(matrix.is_square());

        assert_eq!(*matrix.get(2, 2), 3)
    }
    #[test]
    fn test_dot_product() {
        let matrix_a = GlMatrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);

        let matrix_b = GlMatrix::new(vec![vec![7, 8], vec![9, 10], vec![11, 12]]);

        let matrix_c = matrix_a.dot(&matrix_b);
        print!("{:?}", matrix_c);
        assert_eq!(matrix_c, GlMatrix::new(vec![vec![58, 64], vec![139, 154],]))
    }
    #[test]
    fn test_identity() {
        let matrix = GlMatrix::new(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(GlMatrix::identity(3), matrix);

        let matrix_b = GlMatrix::new(vec![vec![1, 0, 0], vec![0, 5, 0], vec![4, 0, 2]]);

        assert_eq!(matrix_b.dot(&GlMatrix::identity(3)), matrix_b);
    }
    #[test]
    fn test_transpose() {
        let matrix = GlMatrix::new(vec![
            vec![0, 9, 3, 0],
            vec![9, 8, 0, 8],
            vec![1, 8, 5, 3],
            vec![0, 0, 5, 8],
        ]);
        let transpose_matrix = GlMatrix::new(vec![
            vec![0, 9, 1, 0],
            vec![9, 8, 8, 0],
            vec![3, 0, 5, 5],
            vec![0, 8, 3, 8],
        ]);

        assert_eq!(matrix.transpose(), transpose_matrix);

        let transposed_identity: GlMatrix<isize> = GlMatrix::identity(5).transpose();
        assert_eq!(transposed_identity, GlMatrix::identity(5));
    }
    #[test]
    fn test_submatrix() {
        let matrix_1 = GlMatrix::new(vec![vec![1, 5, 0], vec![-3, 2, 7], vec![0, 6, -3]]);

        let submatrix_1 = GlMatrix::new(vec![vec![-3, 2], vec![0, 6]]);

        assert_eq!(matrix_1.submatrix(0, 2), submatrix_1);

        let matrix_2 = GlMatrix::new(vec![
            vec![-6, 1, 1, 6],
            vec![-8, 5, 8, 6],
            vec![-1, 0, 8, 2],
            vec![-7, 1, -1, 1],
        ]);

        let submatrix_2 = GlMatrix::new(vec![vec![-6, 1, 6], vec![-8, 8, 6], vec![-7, -1, 1]]);
        assert_eq!(matrix_2.submatrix(2, 1), submatrix_2);

        let submatrix_3 = GlMatrix::new(vec![vec![3, 5, 0], vec![2, -1, -7], vec![6, -1, 5]]);
        assert_eq!(submatrix_3.minor(1, 0), 25)
    }

    #[test]
    fn test_cofactor() {
        let matrix = GlMatrix::new(vec![vec![3, 5, 0], vec![2, -1, -7], vec![6, -1, 5]]);

        assert_eq!(matrix.cofactor(0, 0), -12);
        assert_eq!(matrix.cofactor(1, 0), -25);
    }

    #[test]
    fn test_det() {
        let matrix_a = GlMatrix::new(vec![vec![1, 2, 6], vec![-5, 8, -4], vec![2, 6, 4]]);

        assert_eq!(matrix_a.cofactor(0, 0), 56);
        assert_eq!(matrix_a.cofactor(0, 1), 12);
        assert_eq!(matrix_a.cofactor(0, 2), -46);

        assert_eq!(matrix_a.det(), -196);

        let matrix_b = GlMatrix::new(vec![
            vec![-2, -8, 3, 5],
            vec![-3, 1, 7, 3],
            vec![1, 2, -9, 6],
            vec![-6, 7, 7, -9],
        ]);

        assert_eq!(matrix_b.cofactor(0, 0), 690);
        assert_eq!(matrix_b.cofactor(0, 1), 447);
        assert_eq!(matrix_b.cofactor(0, 2), 210);
        assert_eq!(matrix_b.cofactor(0, 3), 51);
    }

}
