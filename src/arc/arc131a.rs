/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    a:usize,
    mut b:usize
  }

  let b = if b % 2 != 0 {
    b * 10 / 2
  } else {
    b / 2
  };

  println!("{}0{}", a,b);
  
}