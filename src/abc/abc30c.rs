#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

fn main() {
  input!{
    n: usize,
    m: usize,
    x: usize,
    y: usize,
    val_as: [usize;n],
    val_bs: [usize;m],
  }
  
  let mut count = 0;
  
  let mut ai = 0;
  let mut bi = 0;
  while ai < n {
    let av = val_as[ai] + x;
    
    while bi < m && val_bs[bi] < av {
      bi += 1;
    }
    
    if m <= bi {
      break
    }
    count += 1;
    let bv = val_bs[bi] + y;
    
    while ai < n && val_as[ai] < bv {
      ai += 1;
    }
  }
  
  println!("{}", count);
}