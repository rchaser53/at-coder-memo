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
    mut x: usize,
    mut vals: [usize;n]
  }
  vals.sort();
  
  for i in 0..n {
    x -= vals[i];
  }
  
  println!("{}", n + x / vals[0]);
}