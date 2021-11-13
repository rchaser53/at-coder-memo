#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    k: usize,
    vals: [usize;n]
  }
    
  let mut total = 0;
  let mut ss = 0;
  for i in 0..k {
    ss += vals[i];
    total += vals[i];
  }
  
  for i in k..n {
    ss += vals[i] - vals[i-k];
    total += ss;
  }
  println!("{}", total);
}