/* OUTPUT FILE */
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]

use proconio::{input};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

pub fn main(
) {
  input! {
    n:usize,
    m:usize,
    vals:[(Usize1, Usize1);m]
  }
  
  let mut count = 0;
  let mut g = vec![vec![];n];
  for (l, r) in vals {
    g[l].push(r);
    g[r].push(l);
  }

  let mut result = 0usize;
  for i in 0..n {
    let mut temp = 0;
    for &ti in &g[i] {
      if ti < i {
        temp += 1;
      }
    }
    if temp == 1 {
      result += 1;
    }
  }
  println!("{}", result);
}