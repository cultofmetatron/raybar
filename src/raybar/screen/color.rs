
/*
  this file contains all the stuff related to the canvas
*/
extern crate image;
use super::super::util::{fl_eq};
use std::cmp::{PartialEq};
use std::ops;
use image::{Rgb};

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
  #[allow(dead_code)]
  pub fn black() -> Color {
    Color::new(0.0, 0.0, 0.0)
  }
  #[allow(dead_code)]
  pub fn red() -> Color {
    Color::new(1.0, 0.0, 0.0)
  }
  #[allow(dead_code)]
  pub fn green() -> Color {
    Color::new(0.0, 1.0, 0.0)
  }
  #[allow(dead_code)]
  pub fn blue() -> Color {
    Color::new(0.0, 0.0, 1.0)
  }

  #[allow(dead_code)]
  pub fn to_pixel(self) -> Rgb<u8> {
    image::Rgb([
      (self.red * 256.0).floor() as u8,
      (self.green * 256.0).floor() as u8,
      (self.blue *256.0).floor() as u8
    ])
  }
}

impl PartialEq for Color {
  fn eq(&self, b: &Color) -> bool {
    fl_eq(self.red, b.red) && fl_eq(self.green, b.green) && fl_eq(self.blue, b.blue)
  }

  
}

impl ops::Add<Color> for Color {
  type Output = Color;
  #[allow(dead_code)]
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
  #[allow(dead_code)]
  fn sub(self, rhs: Color) -> Color {
    Color::new(
      self.red - rhs.red,
      self.green - rhs.green,
      self.blue - rhs.blue
    )
  }
}

impl ops::Mul<f64> for Color {
  type Output = Color;
  fn mul(self, rhs: f64) -> Color {
    Color::new(
      self.red * rhs,
      self.green * rhs,
      self.blue * rhs
    )
  }
}

impl ops::Mul<isize> for Color {
  type Output = Color;
  fn mul(self, rhs: isize) -> Color {
    let operant: f64 = rhs as f64;
    Color::new(
      self.red * operant,
      self.green * operant,
      self.blue * operant
    )
  }
}

impl ops::Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color::new(
      self.red * rhs.red,
      self.green * rhs.green,
      self.blue * rhs.blue
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
    #[test]
    fn test_subtract_two_colors() {
      let color1: Color = Color::new(-0.5, 0.4, 1.7);
      let color2: Color = Color::new(0.0, 0.4, 1.7);
      assert_eq!(color1 - color2, Color::new(-0.5, 0.0, 0.0));
    }

    #[test]
    fn test_color_mutiplied_by_scalar() {
      let color: Color = Color::new(-0.5, 0.4, 1.7);
      assert_eq!(color * 2.0, Color::new(-1.0, 0.8, 3.4));
      assert_eq!(color * 2, Color::new(-1.0, 0.8, 3.4));
    }
    #[test]
    fn test_color_mutiplied_by_color() {
      let color1 = Color::new(1.0, 0.2, 0.4);
      let color2 = Color::new(0.9, 1.0, 0.1);
      assert_eq!(color1 * color2, Color::new(0.9, 0.2, 0.04))
    }
}