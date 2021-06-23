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
    vals:[usize;n]
  }

  for v in vals {
    if v % 2 == 0 && v % 3 != 0 && v % 5 != 0 {
      println!("DENIED");
      return
    }
  }
  println!("APPROVED");
  return
}