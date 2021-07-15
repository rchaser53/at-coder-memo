/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    a:usize,
    b:usize
  }

  let mut count = 0;
  let mut now = 1;
  while now < b {
    count += 1;
    now += a - 1;
  }
  println!("{}", count);
}
