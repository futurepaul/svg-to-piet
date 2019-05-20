extern crate svg_piet_macro;

pub use crate::svg_piet_macro::*;

pub fn print_piet(path: &str) {
  for line in pietify::svg_to_strings(path.to_string()) {
    println!("{}", line);
  }
}
