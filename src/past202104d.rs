#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;
 
#[fastout]
fn main() {
  input!{
    n:usize,
    k:usize,
    mut vals:[isize;n]
  }
  
  vals.insert(0, 0);
  let mut memo = vec![0;n+1];
  for i in 1..=n {
    memo[i] = memo[i-1] + vals[i];
  }
  
  for i in 1..=n-k+1 {
    println!("{}", memo[i+k-1] - memo[i-1]);
  }
}