#![feature(trait_alias)]
extern crate num_traits;
//use crate::glprimatives::GlPrimative;
//use raybar::*
mod raybar;
// use raybar::glprimatives::*;
//use raybar::canvas
extern crate clap;
extern crate image;

use clap::{Arg, App, SubCommand};

use image::GenericImageView;

fn main() {
    let matches = App::new("raybar")
        .version("1.0")
        .author("cultofmetatron")
        .about("a toolkit for nonrealtime graphics")
        .arg(Arg::with_name("isomorph")
            .short("iso")
            .long("isomorph")
            .value_name("FILE")
            .help("transforms a top down iso to a isomorphic layout")
            .takes_value(true))
        .get_matches();
    
    let isomorph = matches.value_of("isomorph").unwrap();
    println!("VAluef or isomorph {}", isomorph);

    // pull the file from isomorph and load into memory
    let image = image::open(isomorph).unwrap();
    //let image = 
    //image.
    
    println!("the color is: {:?}", image.color());

}
