/** THIS IS AN OUTPUT FILE. NOT EDIT THIS FILE DIRECTLY. **/
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;
use std::cmp::Ordering;

fn main() {
  input!{
    n:usize,
    x:usize,
    vals:[usize;n]
  }
  let mut count = 0;
  for v in vals {
    if v == x {
      count += 1;
    }
  }
  println!("{}", count);
}