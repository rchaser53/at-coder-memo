/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    a:isize,
    v:isize,
    b:isize,
    w:isize,
    t:isize
  }

  if (a - b).abs() <= (v - w) * t {
    println!("YES");
  } else {
    println!("NO");
  }
}