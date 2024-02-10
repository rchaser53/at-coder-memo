/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    n:usize,
    a:[isize;n]
  }

  let mut now = 0isize;
  for v in a {
    now += v;
    if now < 0 {
      now = 0;
    }
  }
  println!("{}", now);
}