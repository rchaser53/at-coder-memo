#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use petgraph::unionfind::UnionFind;
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
fn main() {
  input!{
    n:usize,
    k:isize,
    vals:[isize;n]
  }
  
  let mut total = 0;
  for v in vals {
    total += std::cmp::min(2 * v, 2 * (k-v).abs());
  }
  println!("{}", total);
}