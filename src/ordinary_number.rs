#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    vals: [usize;n]
  }
  
  let mut count = 0;
  for i in 1..n-1 {
    if vals[i-1] < vals[i] && vals[i] < vals[i+1] {
      count += 1;
    } else if vals[i+1] < vals[i] && vals[i] < vals[i-1] {
      count += 1;
    }
  }
  println!("{}", count);
}