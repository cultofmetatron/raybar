
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
  board: Box<Vec<Box<Vec<Color>>>>
}

impl Canvas {
  #[allow(dead_code)]
  pub fn new(length: usize, width: usize) -> Canvas {
    let board: Vec<Box<Vec<Color>>> = (0..length)
      .into_iter()
      .map(|_x| Canvas::init_row(width))
      .collect();
    Canvas {
      board: Box::new(board)
    }
  }
  fn init_row(size: usize) -> Box<Vec<Color>> {
    let row: Vec<Color> = (0..size)
      .into_iter()
      .map(|_x| Color::black())
      .collect();
    Box::new(row)
  }
  #[allow(dead_code)]
  pub fn set_pixel(mut self, length: usize, width: usize, color: Color) -> Canvas {
    //let mut board = self.board;
    self.board[length][width] = color;
    self
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