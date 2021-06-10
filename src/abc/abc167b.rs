/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    vals:[isize;3],
    mut k:isize
  }

  let mut result = 0isize;
  if k <= vals[0] {
    println!("{}", k);
    return
  }
  k -= vals[0];
  result += vals[0];

  if k <= vals[1] {
    println!("{}", result);
    return
  }
  k -= vals[1];

  println!("{}", result - k);
}