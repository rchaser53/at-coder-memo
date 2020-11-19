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
  
  let mut last = vals[0]; 
  let mut memo = vec![0;n];
  memo[0] = 1;
  for i in 1..n {
    if last < vals[i] {
      memo[i] = memo[i-1] + 1;
    } else {
      memo[i] = 1;
    }
    last = vals[i];
  }
  
  println!("{}", memo.into_iter().sum::<usize>());
}