#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    x: usize,
    vals: [usize;n]
  }
  
  let mut count = 1;
  let mut last = 0;
  for i in 0..n {
    last += vals[i];
    if last <= x {
      count += 1;
    }
  }
  println!("{}", count);
}