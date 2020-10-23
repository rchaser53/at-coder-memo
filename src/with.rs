#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;

#[fastout]
fn main() {
  input! {
    n: usize,
    vals: [isize;n]
  }
  let mut min = isize::max_value();
  for i in -100..=100 {
    let mut temp = 0;
    for ii in 0..n {
      temp += (vals[ii] - i as isize).pow(2);
    }
    min = std::cmp::min(min, temp);
  }
  println!("{}", min);
}