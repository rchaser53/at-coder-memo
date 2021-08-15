/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    ss:[usize;n],
    tt:[usize;n]
  }

  let mut result = vec![0;n];
  for i in 0..n {
    let t = tt[i];
    result[i] = t;
  }

  for i in 0..=2*n {
    let i = i % n;
    let ti = (i+1) % n;
    result[ti] = std::cmp::min(result[i] + ss[i], result[ti]);
  }

  for v in result {
    println!("{}", v);
  }
}
