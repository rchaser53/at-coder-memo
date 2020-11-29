#![allow(unused_imports)]
use proconio::{input, fastout};
use proconio::marker::*;
use std::collections::*;
use std::cmp::Ordering;
use petgraph::unionfind::UnionFind;

const MOD:usize = 1_000_000_007;

fn main() {
  input!{
    a: isize,
    b: isize,
    x: isize,
    y: isize,
  }
 
  let diff = (a - b).abs();
  if diff == 0 {
    println!("{}", x);
    return
  }
 
  if a < b {
    println!("{}", 
      std::cmp::min(
        x + (2*x) * diff,
        x + y * diff
      )  
    );
  } else  {
    if 2 * x < y {
      println!("{}", x + (2*x) * (diff-1));
    } else {
      println!("{}", x + y * (diff-1));
    }
  }
}
