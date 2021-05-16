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
    t: usize,
    vals: [usize;n]
  }
  
  let mut last = vals[0];
  let mut count = t;
  for i in 1..n {
    let diff = vals[i] - last;
    count += std::cmp::min(diff, t);
    last = vals[i];
  }
  
  println!("{}", count);
}
