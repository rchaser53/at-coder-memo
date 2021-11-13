#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    sx: isize,
    sy: isize,
    tx: isize,
    ty: isize
  }
  let diff_x = tx - sx;
  let diff_y = ty - sy;

  for _ in 0..diff_x {
    print!("R");
  }
  for _ in 0..diff_y {
    print!("U");
  }
  for _ in 0..diff_x {
    print!("L");
  }
  for _ in 0..=diff_y {
    print!("D");
  }
  
  for _ in 0..=diff_x {
    print!("R");
  }
  for _ in 0..=diff_y {
    print!("U");
  }
  print!("LU");
  for _ in 0..=diff_x {
    print!("L");
  }
  for _ in 0..=diff_y {
    print!("D");
  }
  print!("R");
}