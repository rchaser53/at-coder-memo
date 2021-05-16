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
    vals: Chars
  }
  
  let mut map = HashMap::new();
  for c in vals {
    *map.entry(c).or_insert(0) += 1;
  }
  
  let mut min = n;
  let mut max = 0;
  let len = map.keys().len();
  for (_, v) in map {
    min = std::cmp::min(min, v);
    max = std::cmp::max(max, v);
  }
  
  if len == 4 {
    println!("{} {}", max, min);
  } else {
    println!("{} 0", max);
  }
}