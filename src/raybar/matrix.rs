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
use std::ops::{Mul, Add, Sub};

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct GlMatrix<T: PartialEq + Ord> {
    content: Vec<Vec<T>>,
}

impl<T: PartialEq + Ord + Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Clone> GlMatrix<T> {
    // new
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
    //pub fn dot(&self, b: GlMatrix<T>) -> {
        
   // }
    #[allow(dead_code)]
    pub fn dot_list<C: PartialEq + Ord + Mul<Output = C> + Add<Output = C> + Sub<Output = C> + Clone>(row: Vec<C>, col: Vec<C>) -> Option<C> {
        if row.len() == 0 {
            return None;  //hack for dealing with null generics
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
            };
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
      let matrix: GlMatrix<i64> = GlMatrix::new(vec![
        vec![1, 2, 3],
        vec![6, 2, 7],
        vec![21, 45, 3]
      ]);
      assert_eq!(matrix.get_dimensions(), (3, 3));
      assert!(matrix.is_square());

      assert_eq!(*matrix.get(2, 2), 3);
    }

}
