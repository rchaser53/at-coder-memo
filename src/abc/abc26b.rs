#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }
  
  let mut result = 0f64;
  vals.sort();
  let mut vals = vals
    .into_iter()
    .map(|v| v as f64)
    .collect::<Vec<f64>>();
  
  for i in 0..n {
    if i % 2 == 0 {
      result += vals[i] * vals[i];
    } else {
      result -= vals[i] * vals[i];
    }
  }
  
  println!("{}", (std::f64::consts::PI * result).abs());
}
