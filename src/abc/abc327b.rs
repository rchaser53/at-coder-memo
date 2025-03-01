/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use std::cmp::Reverse;
use proconio::input;
use proconio::marker::*;
use std::cmp::Ordering;
use std::collections::*;

fn main() {
  input! {
    b:i128
  }
  if b == 1 {
    println!("1");
    return
  }

  for i in 2.. {
    let mut x = i;
    for _ in 1..i {
      x *= i;
    }
    if x == b {
      println!("{}", i);
      return
    }

    if x > b {
      println!("-1");
      return
    }
  }
}