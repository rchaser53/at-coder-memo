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
    n:usize,
    mut vals:[usize;n]
  }
  vals.sort();
  let vals:Vec<f64> = vals.into_iter().map(|v| v as f64).collect::<_>();
  let mid = if n % 2 == 1 {
    vals[n/2]
  } else {
    (vals[n/2-1] + vals[n/2]) / 2f64
  };

  let mut result = 0f64;
  let rmid = mid / 2f64;
  for v in vals {
    result += rmid  + v;
    if v < mid {
      result -= v;
    } else {
      result -= mid;
    }
  }
  println!("{}", result / n as f64);
}