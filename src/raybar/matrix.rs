/*
  matrix primatives for raybar

  todo
  create NxM matrix
  where N -> row
  where M -> col
  where for matrix A, A[n][m] yields the item at row n, col m

*/
use std::cmp::{Ord, PartialEq};
//std::clone::Clone;
//std::copy::{Copy};
use std::ops::{Add, Mul, Sub};
//use std::num::{FpCategory};
extern crate num_traits;

use num_traits::{One, Zero};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct GlMatrix<T: PartialEq + Ord> {
    content: Vec<Vec<T>>,
}

impl<
        T: PartialEq + Ord + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Clone + Copy + Zero + One,
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
    pub fn dot(&self, b: &GlMatrix<T>) -> GlMatrix<T> {
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

    #[allow(dead_code)]
    pub fn dot_list(row: &Vec<T>, col: &Vec<T>) -> Option<T> {
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
        let new_contents: Vec<Vec<T>> = self.content
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
    pub fn minor(&self, row: usize, col: usize) -> T {
        self
            .submatrix(row, col)
            .det()
    }
    #[allow(dead_code)]
    pub fn det(&self) -> T {
        if self.is_square() {
            if self.get_row_size() == 2 {
                self.det_base()
            } else {
                panic!("cannpt comput greater than 2 yet");
            }
        } else {
            panic!("non square matrices have no determinant!");
        }
    }
    #[allow(dead_code)]
    fn det_base(&self) -> T {
        //for the case where the matrix is 2 x 2
        let a = self.content[0][0].clone();
        let d = self.content[1][1].clone();
        let b = self.content[0][1].clone();
        let c = self.content[0][1].clone();
        (a * d) - (b * c)
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
        let matrix_1 = GlMatrix::new(vec![
            vec![1, 5, 0],
            vec![-3, 2, 7],
            vec![0, 6, -3],
        ]);

        let submatrix_1 = GlMatrix::new(vec![
            vec![-3, 2],
            vec![ 0, 6],
        ]);

        assert_eq!(matrix_1.submatrix(0, 2), submatrix_1);

        let matrix_2 = GlMatrix::new(vec![
            vec![-6, 1, 1, 6],
            vec![-8, 5, 8, 6],
            vec![-1, 0, 8, 2],
            vec![-7, 1, -1, 1],
        ]);

        let submatrix_2 = GlMatrix::new(vec![
            vec![-6, 1, 6],
            vec![-8, 8, 6],
            vec![-7, -1, 1],
        ]);
        assert_eq!(matrix_2.submatrix(2, 1), submatrix_2);

        let submatrix_3 = GlMatrix::new(vec![
            vec![3,  5,  0],
            vec![2, -1, -7],
            vec![6, -1,  5],
        ]);
        assert_eq!(submatrix_3.minor(1, 0), 25)
    }

}
