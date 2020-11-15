#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    mut vals: [usize;n]
  }
  
  let mut v = vals.into_iter().enumerate().collect::<Vec<(usize, usize)>>();
  v.sort_by(|a, b| a.1.cmp(&b.1));
  for i in (0..n).rev() {
    println!("{}", v[i].0 + 1);
  }
}