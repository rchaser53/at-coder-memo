/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    h:usize,
    w:usize,
  }

  if h == 1 || w == 1 {
    println!("1");
    return
  }

  let v = h * w;
  if v % 2 == 0 {
    println!("{}", v / 2);
  } else {
    println!("{}", v / 2 + 1);
  }
}