#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    c: i128,
    vals: [(i128, i128);n]
  }
  
  let r0 = vals[0].1 - vals[0].0;
  let l0 = vals[n-1].1 - (c - vals[n-1].0);
  let mut rights = vec![r0;n];
  let mut lefts = vec![l0;n];
  let mut rmaxs = vec![std::cmp::max(r0, 0);n];
  let mut lmaxs = vec![std::cmp::max(l0, 0);n];
  
  for i in 1..n {
    rights[i] = rights[i-1] + vals[i].1 - (vals[i].0 - vals[i-1].0);
    rmaxs[i] = std::cmp::max(rights[i], rmaxs[i-1]);
  }
  
  for i in 1..n {
    lefts[i] = lefts[i-1] + vals[n-1-i].1 - (vals[n-i].0 - vals[n-i-1].0);
    lmaxs[i] = std::cmp::max(lefts[i], lmaxs[i-1]);
  }
  
  let mut max = 0;
  for i in 0..n-1 {
    max = std::cmp::max(max, rights[i] - vals[i].0 + lmaxs[n-i-2]);
    max = std::cmp::max(max, rights[i]);
  }
  max = std::cmp::max(max, rights[n-1]);

  for i in 0..n-1 {
    max = std::cmp::max(max, lefts[i] - (c - vals[n-i-1].0) + rmaxs[n-i-2]);
    max = std::cmp::max(max, lefts[i]);
  }
  max = std::cmp::max(max, lefts[n-1]);

  println!("{}", max);
}