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
    s: isize,
    t: isize,
    mut w: isize,
    vals: [isize;n-1]
  }
  
  let mut count = 0;
  if s <= w && w <= t {
    count += 1;
  }
  
  for v in vals {
    w += v;
    if s <= w && w <= t {
      count += 1;
    }
  }
  println!("{}", count);
}