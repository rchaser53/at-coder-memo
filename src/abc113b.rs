#![allow(unused_imports)]
 
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::HashMap;

const MOD:usize = 1_000_000_007;
 
#[fastout]
fn main() {
  input! {
    n:usize,
    t:f64,
    a:f64,
    vals:[f64;n]
  }
  
  let mut result = 0;
  let mut min = 1_000_000f64;
  for i in 0..n {
    let v = ((t - vals[i] * 0.006f64) - a).abs();
    if v < min {
      result = i;
      min = v; 
    }
  }
  println!("{}", result + 1);
}