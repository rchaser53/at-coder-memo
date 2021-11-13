#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    q: usize,
    vals: [(Usize1, Usize1, usize);q]
  }
    
  for i in 0..n {
    let mut result = 0;
    for ii in (0..q).rev() {
      let (l, r, v) = vals[ii];
      if l <= i && i <= r {
        result = v;
        break
      }
    }
    println!("{}", result);
  }
}