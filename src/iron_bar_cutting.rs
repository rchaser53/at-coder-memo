#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    n: usize,
    vals: [isize;n]
  }
  
  let whole = vals.iter().sum::<isize>();
  let mut sub_total = 0;
  let mut result = isize::max_value();
  for i in 0..n-1 {
    sub_total += vals[i];
    result = std::cmp::min(result, (2 * sub_total - whole).abs());
  }
  println!("{}", result);
}