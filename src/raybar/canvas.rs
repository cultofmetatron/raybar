
/*
  this file contains all the stuff related to the canvas
*/
use super::util::{fl_eq};
use std::cmp::{PartialEq};
use std::ops;

/*
Color is a core primative 

*/

#[derive(Clone, Copy, Debug)]
pub struct Color {
  red: f64,
  green: f64,
  blue: f64
}

impl Color {
  #[allow(dead_code)]
  pub fn new(red: f64, green: f64, blue: f64) -> Color {
    //note: bound to under 1 and over 0
    Color {
      red,
      green,
      blue,
    }
  }
}

impl PartialEq for Color {
  fn eq(&self, b: &Color) -> bool {
    fl_eq(self.red, b.red) && fl_eq(self.green, b.green) && fl_eq(self.blue, b.blue)
  }
}

impl ops::Add<Color> for Color {
  type Output = Color;

  fn add(self, rhs: Color) -> Color {
    Color::new(
      self.red + rhs.red,
      self.green + rhs.green,
      self.blue + rhs.blue
    )
  }
}

impl ops::Sub<Color> for Color {
  type Output = Color;

  fn sub(self, rhs: Color) -> Color {
    Color::new(
      self.red - rhs.red,
      self.green - rhs.green,
      self.blue - rhs.blue
    )
  }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_color() {
      let color: Color = Color::new(-0.5, 0.4, 1.7);
      assert_eq!(color.red, -0.5);
      assert_eq!(color.green, 0.4);
      assert_eq!(color.blue, 1.7);
    }
    #[test]
    fn test_color_equals() {
      let color1: Color = Color::new(-0.5, 0.4, 1.7);
      let color2: Color = Color::new(-0.5, 0.4, 1.7);
      assert_eq!(color1 == color2, true);
    }
    #[test]
    fn test_add_two_colors() {
      let color1: Color = Color::new(-0.5, 0.4, 1.7);
      let color2: Color = Color::new(-0.5, 0.4, 1.7);
      assert_eq!(color1 + color2, Color::new(-1.0, 0.8, 3.4));
    }
}