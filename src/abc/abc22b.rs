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
    vals: [usize;n],
  }
  
  let mut set = HashSet::new();
  let mut count = 0;
  for v in vals {
    if set.contains(&v) {
      count += 1;
    } else {
      set.insert(v);
    }
  }
  
  println!("{}", count);
}