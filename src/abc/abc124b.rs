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
    n: usize,
    vals: [usize;n]
  }

  let mut max = vals[0];
  let mut result = 1;
  for i in 1..n {
    let v = vals[i];
    if max <= v {
      max = v;
      result += 1;
    }
  }
  println!("{}", result);
}
