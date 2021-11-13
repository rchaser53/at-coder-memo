#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: isize,
  }
  
  let mut min = 100_000;
  let mut i = 1;
  while i <= n {
    let vertical = i;
    let horizon = n / i;
    min = std::cmp::min(min, (n / i - i).abs() + n - vertical * horizon);
    i += 1;
  }
  println!("{}", min);
}