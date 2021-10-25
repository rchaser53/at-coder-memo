/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    mut a:usize,
    mut b:usize,
    k:usize
  }

  for i in 0..k {
    if i % 2 == 0 {
      if a % 2 == 1 {
        a -= 1;
      }
      b += a / 2;
      a /= 2;
    } else {
      if b % 2 == 1 {
        b -= 1;
      }
      a += b / 2;
      b /= 2;
    }
  }
  println!("{} {}", a, b);
}