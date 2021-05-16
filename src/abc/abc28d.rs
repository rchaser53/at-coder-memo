#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    n: f64,
    k: f64
  }
  
  let parent = n * n * n;
  let three = 1f64;
  let two = 3f64 * (n - 1f64);
  
  if k == 1f64 || k == n {
    println!("{}", (three + two) / parent);
  } else {
    println!("{}",
      (
        three +
        two + 
        3f64 * 2f64 * (k - 1f64) * 1f64 * (n - k)
      )
        /
      parent
    );  
  }
}
