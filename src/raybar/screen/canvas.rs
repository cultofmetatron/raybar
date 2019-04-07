
use super::color::Color;

use std::vec::Vec;
/*

a Canvas is a core primative for hosling the representation of a screen
it is mutable for perfomrance

new -> returns a black canvas
set_pixel -> 
vector vector pixel(color)

*/
#[allow(dead_code)]
pub struct Canvas {
  board: Vec<Vec<Color>>
}

impl Canvas {
  #[allow(dead_code)]
  pub fn new(length: usize, width: usize) -> Canvas {
    let board: Vec<Vec<Color>> = (0..length)
      .into_iter()
      .map(|_x| Canvas::init_row(width))
      .collect();
    Canvas {
      board: board
    }
  }
  fn init_row(size: usize) -> Vec<Color> {
    let row: Vec<Color> = (0..size)
      .into_iter()
      .map(|_x| Color::black())
      .collect();
    row
  }
  #[allow(dead_code)]
  pub fn set_pixel(mut self, col: usize, row: usize, color: Color) -> Canvas {
    //let mut board = self.board;
    self.board[col][row] = color;
    self
  }
  #[allow(dead_code)]
  pub fn get_pixel(self, col: usize, row: usize) -> Color {
    self.board[col][row].clone()
  }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_canvas() {
      assert_eq!(3, 3);
    }
}