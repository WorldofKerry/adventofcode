pub mod d1;
pub mod d2;
mod d3;
use std::fs;

fn read_input(path: &str) -> String {
    fs::read_to_string(String::from("./src/y2022/") + path).unwrap()
}
