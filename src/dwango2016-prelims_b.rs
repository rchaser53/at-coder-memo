/* OUTPUT FILE */
use proconio::input;
use proconio::marker::*;
use std::collections::*;

pub fn main(
) {
  input! {
    n:usize,
    vals:[usize;n-1],
  }

  let mut result = vec![1;n];
  result[0] = vals[0];

  for i in 0..n-2 {
    let av = vals[i];
    let bv = vals[i+1];

    if av < bv {
      result[i+1] = av;
    } else {
      result[i+1] = bv;
    }
  }
  result[n-1] = vals[n-2];
  println!("{}", result.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
}