/* OUTPUT FILE */
#![allow(unused_imports)]
use proconio::input;
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    n:usize,
    k:usize,
    vals:[usize;n]
  }

  let mut result = 0usize;
  for v in vals {
    if k <= v {
      result += 1;
    }
  }
  println!("{}", result);
}
