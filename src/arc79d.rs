#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;

fn main() {
  input!{
    k:usize
  }
  let n = 50;
  let c = k / n;
  let r = k % n;
  println!("{}", n);
  
  let mut result = vec![0;n];
  for i in 0..n {
    result[i] = 49 + c;
  }
  for i in 0..r {
    result[i] += n + 1;
    for ii in 0..n {
      result[ii] -= 1;
    }
  }
  
  println!("{}", result
    .into_iter()
    .map(|v| v.to_string())
    .collect::<Vec<_>>()
    .join(" ")
  );
}