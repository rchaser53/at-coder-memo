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
    a: usize,
    b: usize,
    k: usize,
    vals: [usize;k]
  }
  
  let mut memo = HashSet::new();
  memo.insert(a);
  memo.insert(b);
  for v in vals {
    if memo.contains(&v) {
      println!("NO");
      return
    }
    memo.insert(v);
  }

  println!("YES");
}
