#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input!{
    n: usize,
    x: isize,
    mut vals: [isize;n]
  }
  
  let mut last = vals[0];
  let mut count = if x < last {
    let v = last - x;
    vals[0] = x;
    last = x;
    v
  } else {
    0
  };
  
  for i in 1..n {
    last += vals[i];
    if x < last {
      let v = last - x;
      count += v;
      vals[i] -= v;
      last -= v;
    }
    last -= vals[i-1];
  }
  println!("{}", count);
}