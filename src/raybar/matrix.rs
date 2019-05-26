/*
  matrix primatives for raybar

  todo
  create NxM matrix
  where N -> row
  where M -> col
  where for matrix A, A[n][m] yields the item at row n, col m

*/
use super::util::fl_eq;
use std::cmp::{Ord, PartialEq};
//std::clone::Clone;
//std::copy::{Copy};
use std::ops::{Add, Mul, Sub};
//use std::num::{FpCategory};
use std::convert::From;
extern crate num_traits;

use num_traits::{Zero, One};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct GlMatrix<T: PartialEq + Ord> {
    content: Vec<Vec<T>>,
}

impl<T: PartialEq + Ord + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Clone + Zero + One>
    GlMatrix<T>
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
        let matrix_a = GlMatrix::new(vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
        ]);

        let matrix_b = GlMatrix::new(vec![
            vec![7,   8],
            vec![9,  10],
            vec![11, 12],
        ]);

        let matrix_c = matrix_a.dot(&matrix_b);
        print!("{:?}", matrix_c);
        assert_eq!(matrix_c, GlMatrix::new(vec![
            vec![58,   64],
            vec![139,   154],
        ]))


    }
    #[test]
    fn test_identity() {
        let matrix = GlMatrix::new(vec![
            vec![1, 0, 0],
            vec![0, 1, 0],
            vec![0, 0, 1]
        ]);
        assert_eq!(GlMatrix::identity(3), matrix);

        let matrix_b = GlMatrix::new(vec![
            vec![1, 0, 0],
            vec![0, 5, 0],
            vec![4, 0, 2]
        ]);
        
        assert_eq!(matrix_b.dot(&GlMatrix::identity(3)), matrix_b);

    }

}
