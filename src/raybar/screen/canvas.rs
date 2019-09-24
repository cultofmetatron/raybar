extern crate image;
use super::color::Color;

use std::vec::Vec;

use image::{ImageBuffer, Rgb};
/*

a Canvas is a core primative for hosling the representation of a screen
it is mutable for perfomrance

new -> returns a black canvas
set_pixel ->
vector vector pixel(color)

*/
#[allow(dead_code)]
pub struct Canvas {
    board: Vec<Vec<Color>>,
}

impl Canvas {
    #[allow(dead_code)]
    pub fn new(length: usize, width: usize) -> Canvas {
        let board: Vec<Vec<Color>> = (0..length)
            .into_iter()
            .map(|_x| Canvas::init_row(width))
            .collect();
        Canvas { board: board }
    }
    fn init_row(size: usize) -> Vec<Color> {
        let row: Vec<Color> = (0..size).into_iter().map(|_x| Color::black()).collect();
        row
    }
    #[allow(dead_code)]
    pub fn set_pixel(&mut self, col: usize, row: usize, color: Color) -> &Canvas {
        //let mut board = self.board;
        self.board[col][row] = color;
        self
    }
    #[allow(dead_code)]
    pub fn get_pixel(&self, col: usize, row: usize) -> Color {
        self.board[col][row].clone()
    }
    #[allow(dead_code)]
    pub fn get_dimensions(&self) -> (usize, usize) {
        let length = self.board.len();
        if length == 0 {
            (0, 0)
        } else {
            (length, self.board[0].len())
        }
    }
    // image::ImageBuffer<image::Rgb<u8>, Vec<image::Rgb<u8>::Subpixel>>
    #[allow(dead_code)]
    pub fn export_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let (length, width) = self.get_dimensions();
        let mut buffer = image::ImageBuffer::new(length as u32, width as u32);

        // for each of the pixels in the buffer, copy over the coresponding version
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            *pixel = self.get_pixel(x as usize, y as usize).to_pixel();
        }
        buffer
    }
    #[allow(dead_code)]
    pub fn save(&self, filename: String) {
        let buffer = self.export_buffer();
        buffer.save(filename).unwrap();
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
