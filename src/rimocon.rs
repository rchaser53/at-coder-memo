#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
  input!{
    a: isize,
    b: isize,
  }
  
  let mut diff = (a - b).abs() as usize;
  let mut memo = vec![0,
    1, 2, 3, 2, 1,	// 1, 2, 3, 4, 5
    2, 3, 3, 2, 1 	// 6, 7, 8, 9, 10
  ];
  
  if 10 <= diff {
    println!("{}", diff / 10 + memo[diff % 10]);
  } else {
    println!("{}", memo[diff % 10]);
  }
}