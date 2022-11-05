#![allow(dead_code)]
#![allow(unused)]

use math::{constants::FOUR, Matrix4x4};

mod math;

fn main() {
    let row = Matrix4x4::new_row_major();
    let row2 = Matrix4x4::new_row_major();
    println!("row major ::  {:?}", row);
    println!("col major ::  {:?}", row2);
    println!("col major ::  {:?}", row * row2);
}
