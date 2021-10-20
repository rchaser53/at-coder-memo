use proconio::input;
use proconio::marker::*;
use std::collections::*;
use std::cmp::Reverse;

fn main() {
  input! {
    a:usize,
    b:usize,
    mut c:usize
  }

  if c <= a + b {
    println!("{}", b + c);
  } else {
    println!("{}", a+b*2+1);
  }
}