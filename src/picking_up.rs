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
    vals: [(isize,isize);n]
  }

  let mut map = HashMap::new();
  for i in 0..n {
    let (x1, y1) = vals[i];
    for ii in 0..n {
      if i == ii { continue }
      let (x2, y2) = vals[ii];
      *map.entry((x1-x2, y1-y2)).or_insert(0) += 1;
    }
  }
  
  let mut min = n;
  for (_, val) in map {
    min = std::cmp::min(min, n-val);
  }
  
  println!("{}", min);
}