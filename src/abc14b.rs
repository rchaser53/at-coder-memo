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
    x: usize,
    vals: [usize;n]
  }
  
  let mut count = 0;
  for i in 0..n {
    let v = 1 << i;
    if v & x == v {
      count += vals[i];
    }
  }
  println!("{}", count);
}