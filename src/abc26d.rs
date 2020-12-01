#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    a: f64,
    b: f64,
    c: f64
  }
  
  let mut left = 0f64;
  let mut right = 1000f64;
  let mut mid = 0f64;
  for _ in 0..200 {
    mid = (left + right) / 2f64;
    let v = a * mid + b * (c * mid * std::f64::consts::PI).sin();
    if v < 100f64 {
      left = mid;
    } else {
      right = mid;
    }
  }

  println!("{}", right);
}
