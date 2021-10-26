/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    n:usize,
    h:usize,
    w:usize,
    vals:[(usize,usize);n]
  }

  let mut result = 0;
  for (hv, wv) in vals {
    if h <= hv && w <= wv {
      result += 1;
    }
  }
  println!("{}", result);
}