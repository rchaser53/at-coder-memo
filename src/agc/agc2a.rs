/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    a:isize,
    b:isize
  }
  
  if 0 < a {
    println!("Positive");
  } else if a == 0 {
    println!("Zero");
  } else if a < 0 {
    if 0 <= b {
      println!("Zero");
    } else {
      if ((b - a).abs() + 1) % 2 == 0 {
        println!("Positive");
      } else {
        println!("Negative");
      }
    }
  }
}
