/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    a:isize,
    b:isize,
    w:isize
  }
  let mut min = 1000010;
  let mut max = -1;
  let w = 1000 * w;
  for i in 1..1000010 {
    if a * i <= w && w <= b * i {
      min = std::cmp::min(min, i);
      max = std::cmp::max(max, i);
    }
  }

  if min == 1000010 || max == -1 {
    println!("UNSATISFIABLE");
  } else {
    println!("{} {}", min, max);
  }
}