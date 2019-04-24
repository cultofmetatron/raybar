/*
  matrix primatives for raybar

  todo
  create NxM matrix
  where N -> row
  where M -> col
  where for matrix A, A[n][m] yields the item at row n, col m

*/
use std::cmp::{PartialEq, Ord};
use std::ops;
use super::util::{fl_eq};

#[allow(dead_code)]
pub struct GlMatrix<T: PartialEq + Ord> {
  content: Vec<Vec<T>>
}

impl<T: PartialEq + Ord> GlMatrix<T> {

  // new
  #[allow(dead_code)]
  pub fn new(content: Vec<Vec<T>>) -> GlMatrix<T> {
    // for each sup vector, the length must be consitent
    if GlMatrix::vec_sizing(&content) {
      GlMatrix {
        content: content
      }
    } else {
      panic!("all rows must be the same size!");
    }
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
      },
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