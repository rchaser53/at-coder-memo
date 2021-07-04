/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n: usize,
    vals: [(usize,usize);n]
  }

  let mut exps = vec![0f64;101];
  let mut exp_sums = vec![0f64;101];
  let mut result = 0f64;
  for i in 0..n-1 {
    let (l, r) = vals[i];
    let base_rate = 1f64 / (r - l + 1) as f64;
    for j in l..=r {
      exps[j] += base_rate;
    }
    for j in 1..=100 {
      exp_sums[j] = exp_sums[j-1] + exps[j];
    }

    let (l1, r1) = vals[i+1];
    let next_rate = 1f64 / (r1 - l1 + 1) as f64;
    for j in l1..=r1 {
      result += next_rate * ((i+1) as f64 - exp_sums[j]);
    }
  }
  println!("{}", result);
}