#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input! {
    x: usize,
  }
  
  let mut count = 0;
  let mut total = 0;
  let mut i = 1;
  while total < x {
    total += i;
    count += 1;
    i += 1;
  }
  println!("{}", count);
}